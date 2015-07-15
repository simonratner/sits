use iup;
use iup::prelude::*;
use iup::control::{Frame, List, Text};
use iup::led;

use prop::{Property, PropertyMap, PropertyMapRef};

// LED dialog specification.
static DIALOG: &'static str = r#"
    text_emeralds = text[SPIN=YES, SPINMAX=99999, MASKINT=0:99999, ALIGNMENT=ARIGHT](text_emeralds_edited)
    text_int = text[SPIN=YES, SPINMAX=99, MASKINT=0:99, ALIGNMENT=ARIGHT](text_int_edited)
    text_dex = text[SPIN=YES, SPINMAX=99, MASKINT=0:99, ALIGNMENT=ARIGHT](text_dex_edited)
    text_str = text[SPIN=YES, SPINMAX=99, MASKINT=0:99, ALIGNMENT=ARIGHT](text_str_edited)
    text_occ = text[SPIN=YES, SPINMAX=99, MASKINT=0:99, ALIGNMENT=ARIGHT](text_occ_edited)
    text_per = text[SPIN=YES, SPINMAX=99, MASKINT=0:99, ALIGNMENT=ARIGHT](text_per_edited)

    list_party = list[DROPDOWN=YES, VALUE=1](list_party_changed)

    dlg = dialog(
        vbox[CGAP=2, CMARGIN=4x2, ALIGNMENT=ARIGHT](
            gridbox[NUMDIV=2, SIZECOL=1, CGAPCOL=4, CGAPLIN=2, NORMALIZESIZE=HORIZONTAL](
                label("Emeralds"), text_emeralds,
                label("Party"), list_party
            ),
            member = frame[TITLE=""](
                gridbox[NUMDIV=2, SIZECOL=1, CGAPCOL=4, CGAPLIN=2, NORMALIZESIZE=HORIZONTAL](
                    label("Intelligence"), text_int,
                    label("Dexterity"),    text_dex,
                    label("Strength"),     text_str,
                    label("Occult"),       text_occ,
                    label("Perception"),   text_per
                )
            )
        )
    )
"#;

fn from_name<E: Element>(name: &str) -> E {
    E::from_handle(E::from_name(name).unwrap()).unwrap()
}

fn bind<E: Element + ValueChangedCb>(elem: &mut E, props: PropertyMapRef, key: &'static str) {
    match props.borrow().get(key) {
        Some(&Property::Integer(v)) => {
            elem.set_attrib("VALUE", v.to_string());
        },
        _ => {}
    };
    let props_clone = props.clone();
    elem.set_valuechanged_cb(move |(elem,): (E,)| {
        let val = elem.attrib("VALUE").unwrap().parse::<u32>().unwrap();
        props_clone.borrow_mut().insert(key.to_string(), Property::Integer(val));
    });
}

pub fn run_ui_loop(game: PropertyMapRef, party: &[PropertyMapRef]) {
    iup::with_iup(|| {
        // See also led::load(path) to load from a file
        led::load_buffer(DIALOG).unwrap();

        let mut text_emeralds = from_name::<Text>("text_emeralds");
        bind(&mut text_emeralds, game.clone(), "Emeralds");

        let mut dlg = from_name::<Dialog>("dlg");
        dlg.show()
    }).unwrap();
}
