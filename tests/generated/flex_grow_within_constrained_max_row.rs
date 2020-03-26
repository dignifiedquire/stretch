#[test]
fn flex_grow_within_constrained_max_row() {
    let mut stretch = stretch::Stretch::new();
    let node00: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                flex_shrink: 1f32,
                flex_basis: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node01: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50f32), ..Default::default() },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node0: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node00, node01],
        )
        .unwrap();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(&node).unwrap().size.width, 200f32);
    assert_eq!(stretch.layout(&node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().size.width, 67f32);
    assert_eq!(stretch.layout(&node00).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node01).unwrap().size.width, 33f32);
    assert_eq!(stretch.layout(&node01).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node01).unwrap().location.x, 67f32);
    assert_eq!(stretch.layout(&node01).unwrap().location.y, 0f32);
}
