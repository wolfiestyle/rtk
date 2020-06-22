#![allow(dead_code)]
use widgets::widget::WidgetId;
use widgets_derive::ObjectId;

#[derive(ObjectId)]
struct TestStruct {
    val: i32,
    my_id: WidgetId,
    stuff: String,
}

#[derive(ObjectId)]
struct TestTuple(i32, WidgetId, String);

#[derive(ObjectId)]
enum TestEnum {
    Struct { stuff: WidgetId, asd: i32 },
    Tuple(WidgetId, i32),
    Foo(TestStruct),
    Bar { item: TestTuple, val: i32 },
}

#[test]
fn object_id() {
    use widgets::widget::ObjectId;

    let s = TestStruct {
        val: 42,
        my_id: WidgetId::new(),
        stuff: "foo".into(),
    };
    assert_eq!(s.get_id(), s.my_id);

    let t = TestTuple(42, WidgetId::new(), "foo".into());
    assert_eq!(t.get_id(), t.1);

    let ids = [WidgetId::new(), WidgetId::new(), s.my_id, t.1];
    let e1 = TestEnum::Struct { asd: 42, stuff: ids[0] };
    let e2 = TestEnum::Tuple(ids[1], 33);
    let e3 = TestEnum::Foo(s);
    let e4 = TestEnum::Bar { item: t, val: 13 };
    assert_eq!(e1.get_id(), ids[0]);
    assert_eq!(e2.get_id(), ids[1]);
    assert_eq!(e3.get_id(), ids[2]);
    assert_eq!(e4.get_id(), ids[3]);
}
