#[test]
fn margin_should_not_be_part_of_max_height() {
    let mut stretch = stretch::Stretch::new();
    let node0: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect { top: stretch::style::Dimension::Points(20f32), ..Default::default() },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(250f32),
                    height: stretch::style::Dimension::Points(250f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(&node).unwrap().size.width, 250f32);
    assert_eq!(stretch.layout(&node).unwrap().size.height, 250f32);
    assert_eq!(stretch.layout(&node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.y, 20f32);
}
