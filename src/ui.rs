use std::cell::RefCell;
use std::fs::copy;
use std::path::Path;
use std::str::FromStr;
use std::rc::Rc;

use iup;
use iup::prelude::*;
use iup::control::{Button, Frame, List, Text};
use iup::dialog::{FileDlg};
use iup::led;

use time;

use parser::{read_path, write_path};
use property::{Property, PropertyMap};

// Since we need to share mutable state with 'static ui callbacks,
// we clone a refcounted cell for moving into each callback.
type PropertyMapRc = Rc<RefCell<PropertyMap>>;

// LED dialog specification.
static DIALOG: &'static str = r#"
    text_emeralds = text[SPIN=YES, SPINMAX=99999, MASKINT=0:99999, ALIGNMENT=ARIGHT](_)
    text_int = text[SPIN=YES, SPINMIN=-9, SPINMAX=99, MASKINT=-9:99, ALIGNMENT=ARIGHT](_)
    text_dex = text[SPIN=YES, SPINMIN=-9, SPINMAX=99, MASKINT=-9:99, ALIGNMENT=ARIGHT](_)
    text_str = text[SPIN=YES, SPINMIN=-9, SPINMAX=99, MASKINT=-9:99, ALIGNMENT=ARIGHT](_)
    text_occ = text[SPIN=YES, SPINMIN=-9, SPINMAX=99, MASKINT=-9:99, ALIGNMENT=ARIGHT](_)
    text_per = text[SPIN=YES, SPINMIN=-9, SPINMAX=99, MASKINT=-9:99, ALIGNMENT=ARIGHT](_)

    list_party = list[DROPDOWN=YES, VALUE=1, VISIBLE_ITEMS=6](_)

    button_save = button[PADDING=6x1]("Save and Close", _)
    button_cancel = button[PADDING=6x1]("Cancel", _)

    dlg_open = filedlg[TITLE="Select saved game folder:", DIALOGTYPE=DIR]()

    dlg = dialog[TITLE="SitS Editor"](
        vbox[CGAP=2, CMARGIN=4x2, ALIGNMENT=ARIGHT](
            gridbox[NUMDIV=2, SIZECOL=1, CGAPCOL=4, CGAPLIN=2, NORMALIZESIZE=HORIZONTAL](
                label("Emeralds"), text_emeralds,
                label("Party"), list_party
            ),
            member = frame[TITLE=""](
                gridbox[NUMDIV=2, SIZECOL=1, CGAPCOL=4, CGAPLIN=2, NORMALIZESIZE=HORIZONTAL](
                    label("Intelligence (+10)"), text_int,
                    label("Dexterity (+10)"),    text_dex,
                    label("Strength (+10)"),     text_str,
                    label("Occult (+10)"),       text_occ,
                    label("Perception (+10)"),   text_per
                )
            ),
            hbox[CGAP=2](
                button_save,
                button_cancel
            )
        )
    )
"#;

// Get an element from named handle (defined in the LED input)
fn from_name<E>(name: &str) -> E where E: Element {
    E::from_handle(E::from_name(name).unwrap()).unwrap()
}

// Data-bind an element to a numeric property value.
//
// Value of the element is set to the current value of the property, and
// changes to element value are written back to the property map.
//
// @param props {PropertyMapRc} a cloned refcounted property map.
//
fn bind<T, E>(elem: &mut E, props: PropertyMapRc, key: &'static str)
    where Property: From<T>,
          T: FromStr + Default,
          E: Element + ValueChangedCb {

    // Remove previous bindings, if any.
    elem.remove_valuechanged_cb();

    if let Some(ref prop) = props.borrow().get(key) {
        elem.set_attrib("VALUE", prop.to_string());
    }
    elem.set_valuechanged_cb(move |(elem,): (E,)| {
        if let Some(ref value) = elem.attrib("VALUE") {
            if let Ok(v) = T::from_str(value) {
                props.borrow_mut().insert(key.to_string(), Property::from(v));
            } else {
                props.borrow_mut().insert(key.to_string(), Property::from(T::default()));
            }
        }
    });
}

macro_rules! bind_stat {
    ($i:ident, $p:expr, $e:expr) => {
        bind::<f32,_>(&mut from_name::<Text>(stringify!($i)), $p.clone(), $e);
    }
}

// Data-bind all elements relevant to a party member.
//
// @param props {PropertyMapRc} a cloned refcounted property map.
//
fn bind_member(props: PropertyMapRc) {
    if let Some(&Property::String(ref name)) = props.borrow().get("Name") {
        if let Some(&Property::Float(level)) = props.borrow().get("Level") {
            let title = format!("{} (Level {})", name, level);
            from_name::<Frame>("member").set_attrib("TITLE", title);
        }
    }
    bind_stat!(text_int, props, "Int");
    bind_stat!(text_dex, props, "Dex");
    bind_stat!(text_str, props, "Str");
    bind_stat!(text_occ, props, "Occ");
    bind_stat!(text_per, props, "Per");
}

/// Ui entry point.
///
/// Starts by showing a direction selection dialog; after the user selects a directory,
/// the game is loaded from that directory and values bound to the ui elements.
///
pub fn ui_loop() -> Result<(), String> {
    match iup::with_iup(|| {
        // See also led::load(path) to load from a file
        led::load_buffer(DIALOG).unwrap();

        // Select saved game location
        let mut dlg_open = from_name::<FileDlg>("dlg_open");
        let dir = match dlg_open.popup(DialogPos::CenterParent, DialogPos::CenterParent) {
            Ok(..) => match dlg_open.attrib("STATUS") {
                Some(ref s) if s == "0" => {
                    dlg_open.attrib("VALUE").unwrap()
                },
                _ => return Err("File selection cancelled.".to_string())
            },
            _ => return Err("File selection failed.".to_string())
        };

        // Read game and party member files
        let game = Rc::new(RefCell::new({
            let path = Path::new(&dir).join("Game.txt");
            match read_path(path.as_path()) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!("Cannot read {:?}: {}", path, e))
                }
            }
        }));
        let party = Rc::new(RefCell::new({
            let mut members: Vec<PropertyMapRc> = Vec::new();
            if let Some(&Property::String(ref ids)) = game.borrow().get("PartyIDs") {
                for id in ids.split(",") {
                    let path = Path::new(&dir).join("Party".to_string() + id + ".txt");
                    match read_path(path.as_path()) {
                        Ok(v) => members.push(Rc::new(RefCell::new(v))),
                        Err(e) => {
                            return Err(format!("Cannot read {:?}: {}", path, e))
                        }
                    };
                }
            }
            members
        }));

        // Write game and party member files on save
        let mut button_save = from_name::<Button>("button_save");
        {
            let game_clone = game.clone();
            let party_clone = party.clone();
            button_save.set_action(move |_| {
                let timestamp = time::strftime("%FT%H.%M.%SZ.txt", &time::now_utc()).unwrap();
                let path = Path::new(&dir).join("Game.txt");
                copy(path.as_path(), path.with_extension(&timestamp)).unwrap();
                write_path(path.as_path(), &game_clone.borrow()).unwrap();
                for member in party_clone.borrow().iter().skip(1) {
                    if let Some(&Property::String(ref id)) = member.borrow().get("PartyID") {
                        let path = Path::new(&dir).join("Party".to_string() + id + ".txt");
                        copy(path.as_path(), path.with_extension(&timestamp)).unwrap();
                        write_path(path.as_path(), &member.borrow()).unwrap();
                    }
                }
                CallbackReturn::Close
            });
        }
        let mut button_cancel = from_name::<Button>("button_cancel");
        button_cancel.set_action(|_| {
            CallbackReturn::Close
        });

        let mut text_emeralds = from_name::<Text>("text_emeralds");
        bind::<u32,_>(&mut text_emeralds, game, "Emeralds");

        let mut list_party = from_name::<List>("list_party");
        let mut list_party_items: Vec<String> = Vec::new();
        for member in party.borrow().iter().skip(1) {
            if let Some(&Property::String(ref v)) = member.borrow().get("Name") {
                list_party_items.push(v.to_string());
            }
        }
        let party_clone = party.clone();
        list_party.set_items(list_party_items);
        list_party.set_action(move |(_, _, i, _)| {
            let member = party_clone.borrow()[i as usize].clone();
            bind_member(member);
        });
        bind_member(party.borrow()[1].clone());

        let mut dlg = from_name::<Dialog>("dlg");
        dlg.show()

    }) {
        Err(iup::InitError::UserError(s)) => Err(s),
        Err(e) => Err(format!("{:?}", e)),
        _ => Ok(())
    }
}
