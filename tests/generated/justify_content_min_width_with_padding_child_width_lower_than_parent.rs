#[test]
fn justify_content_min_width_with_padding_child_width_lower_than_parent() {
    let mut stretch = stretch::Stretch::new();
    let node000: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(199f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node00: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                justify_content: stretch::style::JustifyContent::Center,
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(400f32),
                    ..Default::default()
                },
                padding: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(100f32),
                    end: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node000],
        )
        .unwrap();
    let node0: stretch::node::StretchNode =
        stretch.new_node(stretch::style::Style { ..Default::default() }, vec![node00]).unwrap();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(1080f32),
                    height: stretch::style::Dimension::Points(1584f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(&node).unwrap().size.width, 1080f32);
    assert_eq!(stretch.layout(&node).unwrap().size.height, 1584f32);
    assert_eq!(stretch.layout(&node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.width, 1080f32);
    assert_eq!(stretch.layout(&node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().size.width, 400f32);
    assert_eq!(stretch.layout(&node00).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(&node00).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(&node000).unwrap().size.width, 199f32);
    assert_eq!(stretch.layout(&node000).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(&node000).unwrap().location.x, 101f32);
    assert_eq!(stretch.layout(&node000).unwrap().location.y, 0f32);
}
