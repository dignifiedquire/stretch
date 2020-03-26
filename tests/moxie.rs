#![feature(track_caller)]

#[cfg(test)]
mod moxie_node {
    use stretch::geometry::*;
    use stretch::id::Id;
    use stretch::node::{MeasureFunc, Node, Stretch};
    use stretch::result::*;
    use stretch::style::*;
    use stretch::{NodeData, StretchNodeData};

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    struct MoxieNode {
        instance: Id,
        local: Id,
    }

    impl Node for MoxieNode {
        type Data = StretchNodeData;
        fn new(instance: Id, local: Id) -> Self {
            MoxieNode { instance, local }
        }

        fn local(&self) -> Id {
            self.local
        }
    }

    #[derive(Debug)]
    struct MoxieNodeData {}

    impl NodeData for MoxieNodeData {
        fn new(style: Style) -> Self {
            todo!()
        }
        fn new_leaf(style: Style, measure: MeasureFunc) -> Self {
            todo!()
        }

        fn style(&self) -> &Style {
            todo!()
        }
        fn style_mut(&mut self) -> &mut Style {
            todo!()
        }
        fn measure(&self) -> Option<&MeasureFunc> {
            todo!()
        }
        fn measure_mut(&mut self) -> &mut Option<MeasureFunc> {
            todo!()
        }
        fn layout(&self) -> &Layout {
            todo!()
        }
        fn layout_mut(&mut self) -> &mut Layout {
            todo!()
        }
        fn layout_cache(&self) -> Option<&Cache> {
            todo!()
        }
        fn layout_cache_mut(&mut self) -> &mut Option<Cache> {
            todo!()
        }
        fn is_dirty(&self) -> bool {
            todo!()
        }
        fn set_dirty(&mut self, dirty: bool) {
            todo!()
        }
    }

    use std::cell::RefCell;

    #[derive(Debug)]
    struct App {
        children: MoxieNode,
        layout: Layout,
    }

    struct StretchEnv {
        stretch: RefCell<Stretch<MoxieNode>>,
    }
    impl std::fmt::Debug for StretchEnv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StretchEnv")
        }
    }

    impl StretchEnv {
        fn new() -> Self {
            StretchEnv { stretch: RefCell::new(Stretch::new()) }
        }
    }

    #[topo::nested]
    fn app() -> App {
        illicit::child_env!(
            StretchEnv => StretchEnv::new()
        )
        .enter(|| {
            topo::call(|| {
                dbg!("CALLED");
                let stretch = illicit::Env::expect::<StretchEnv>();
                let mut s = stretch.stretch.borrow_mut();

                let child1: MoxieNode =
                    s.new_leaf(Style::default(), Box::new(|_| Ok(Size { width: 200.0, height: 200.0 }))).unwrap();
                let child2: MoxieNode =
                    s.new_leaf(Style::default(), Box::new(|_| Ok(Size { width: 200.0, height: 200.0 }))).unwrap();
                let node: MoxieNode = s.new_node(Style::default(), vec![child1, child2]).unwrap();

                s.set_measure(&node, Some(Box::new(|_| Ok(Size { width: 100.0, height: 100.0 })))).unwrap();
                s.compute_layout(&node, Size::undefined()).unwrap();

                let layout = s.layout(&node).unwrap().clone();

                App { children: node, layout }
            })
        })
    }

    #[test]
    fn children() {
        let mut runtime = moxie::embed::Runtime::new(app);

        let out = runtime.run_once();
        dbg!(&out);

        let out = runtime.run_once();
        dbg!(&out);
    }
}
