use time::format_description::{modifier, Component, FormatItem, OwnedFormatItem};

#[test]
fn borrowed_format_item_component_conversions() {
    let component = Component::Year(modifier::Year::default());
    let item = FormatItem::from(component);
    assert!(matches!(item, FormatItem::Component(inner) if inner == component));
    assert_eq!(Component::try_from(item), Ok(component));
    assert!(Component::try_from(FormatItem::Literal(b"")).is_err());
    assert!(<&[FormatItem<'_>]>::try_from(FormatItem::Literal(b"")).is_err());
}

#[test]
fn borrowed_format_item_compound_conversions() {
    let compound = [FormatItem::Literal(b"")].as_slice();
    let item = FormatItem::from(compound);
    assert!(matches!(item, FormatItem::Compound(inner) if inner == compound));
    assert_eq!(<&[FormatItem<'_>]>::try_from(item), Ok(compound));
}

#[test]
fn borrowed_format_item_equality() {
    let component = Component::Year(modifier::Year::default());
    let compound = [FormatItem::Literal(b"")].as_slice();
    let component_item = FormatItem::from(component);
    let compound_item = FormatItem::from(compound);

    assert_eq!(component, component_item);
    assert_eq!(component_item, component);
    assert_eq!(compound, compound_item);
    assert_eq!(compound_item, compound);
}

#[test]
fn owned_format_item_component_conversions() {
    let component = Component::Year(modifier::Year::default());
    let item = OwnedFormatItem::from(component);
    assert!(matches!(item, OwnedFormatItem::Component(inner) if inner == component));
    assert_eq!(Component::try_from(item), Ok(component));
    assert!(Component::try_from(OwnedFormatItem::Literal(Box::new([]))).is_err());
    assert!(Vec::<OwnedFormatItem>::try_from(OwnedFormatItem::Literal(Box::new([]))).is_err());
}

#[test]
fn owned_format_item_compound_conversions() {
    let compound = vec![OwnedFormatItem::Literal(Box::new([]))];
    let item = OwnedFormatItem::from(compound.clone());
    assert!(matches!(item.clone(), OwnedFormatItem::Compound(inner) if inner.to_vec() == compound));
    assert_eq!(Vec::<OwnedFormatItem>::try_from(item), Ok(compound));
}

#[test]
fn owned_format_item_equality() {
    let component = Component::Year(modifier::Year::default());
    let compound = OwnedFormatItem::from([FormatItem::Literal(b"")].as_slice());
    let component_item = OwnedFormatItem::from(component);

    assert_eq!(component, component_item);
    assert_eq!(component_item, component);
    assert_eq!(
        compound,
        [OwnedFormatItem::Literal(Box::new([]))].as_slice()
    );
    assert_eq!(
        [OwnedFormatItem::Literal(Box::new([]))].as_slice(),
        compound
    );
}
