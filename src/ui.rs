use std::cell::RefCell;
use std::str::FromStr;
use std::rc::Rc;

use iup;
use iup::prelude::*;
use iup::control::{Frame, List, Text};
use iup::led;

use prop::{Property, PropertyMap, PropertyMapRef};

// LED dialog specification.
static DIALOG: &'static str = r#"
    text_emeralds = text[SPIN=YES, SPINMAX=99999, MASKINT=0:99999, ALIGNMENT=ARIGHT](_)
    text_int = text[SPIN=YES, SPINMIN=-9, SPINMAX=99, MASKINT=-9:99, ALIGNMENT=ARIGHT](_)
    text_dex = text[SPIN=YES, SPINMIN=-9, SPINMAX=99, MASKINT=-9:99, ALIGNMENT=ARIGHT](_)
    text_str = text[SPIN=YES, SPINMIN=-9, SPINMAX=99, MASKINT=-9:99, ALIGNMENT=ARIGHT](_)
    text_occ = text[SPIN=YES, SPINMIN=-9, SPINMAX=99, MASKINT=-9:99, ALIGNMENT=ARIGHT](_)
    text_per = text[SPIN=YES, SPINMIN=-9, SPINMAX=99, MASKINT=-9:99, ALIGNMENT=ARIGHT](_)

    list_party = list[DROPDOWN=YES, VALUE=1](_)

    dlg = dialog(
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
// @param props {PropertyMapRef} a cloned refcounted property map.
//
fn bind<T, E>(elem: &mut E, props: PropertyMapRef, key: &'static str)
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
            println!("{:?}", props.borrow().get(key));
        }
    });
}

// Data-bind all elements relevant to a party member.
//
// @param props {PropertyMapRef} a cloned refcounted property map.
//
fn bind_member(props: PropertyMapRef) {
    if let Some(&Property::String(ref v)) = props.borrow().get("Name") {
        from_name::<Frame>("member").set_attrib("TITLE", v.to_string());
    }
    let mut text_int = from_name::<Text>("text_int");
    bind::<f32,_>(&mut text_int, props.clone(), "Int");

    let mut text_dex = from_name::<Text>("text_dex");
    bind::<f32,_>(&mut text_dex, props.clone(), "Dex");

    let mut text_str = from_name::<Text>("text_str");
    bind::<f32,_>(&mut text_str, props.clone(), "Str");

    let mut text_occ = from_name::<Text>("text_occ");
    bind::<f32,_>(&mut text_occ, props.clone(), "Occ");

    let mut text_per = from_name::<Text>("text_per");
    bind::<f32,_>(&mut text_per, props.clone(), "Per");
}

pub fn run_ui_loop(game: PropertyMapRef, party: Rc<RefCell<Vec<PropertyMapRef>>>) {
    iup::with_iup(|| {
        // See also led::load(path) to load from a file
        led::load_buffer(DIALOG).unwrap();

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
    }).unwrap();
}
