#![allow(dead_code)]
use widgets::geometry::{Position, Rect, Size};
use widgets_derive::Bounds;

#[derive(Bounds)]
struct TestStruct1 {
    val: i32,
    my_rect: Rect,
    stuff: String,
}

#[derive(Bounds)]
struct TestStruct2 {
    val: i32,
    pos: Position,
    size: Size,
}

#[derive(Bounds)]
struct TestTuple1(i32, Rect, String);

#[derive(Bounds)]
struct TestTuple2(i32, Size, Position);

#[derive(Bounds)]
enum TestEnum {
    First(TestStruct1),
    Second(TestTuple2),
    Third(Rect),
}

#[test]
fn bounds() {
    use widgets::geometry::Bounds;

    let rect = Rect::new([0, 1], [20, 30]);
    let pos = Position::new(12, 34);
    let size = Size::new(320, 240);

    let s1 = TestStruct1 {
        val: 42,
        my_rect: rect,
        stuff: "foo".into(),
    };
    assert_eq!(s1.get_position(), rect.pos);
    assert_eq!(s1.get_size(), rect.size);
    assert_eq!(s1.get_bounds(), rect);

    let s2 = TestStruct2 { val: 33, pos, size };
    assert_eq!(s2.get_position(), pos);
    assert_eq!(s2.get_size(), size);
    assert_eq!(s2.get_bounds(), Rect::new(pos, size));

    let t1 = TestTuple1(42, rect, "foo".into());
    assert_eq!(t1.get_position(), rect.pos);
    assert_eq!(t1.get_size(), rect.size);
    assert_eq!(t1.get_bounds(), rect);

    let t2 = TestTuple2(33, size, pos);
    assert_eq!(t2.get_position(), pos);
    assert_eq!(t2.get_size(), size);
    assert_eq!(t2.get_bounds(), Rect::new(pos, size));

    let e1 = TestEnum::First(s1);
    let e2 = TestEnum::Second(t2);
    let e3 = TestEnum::Third(rect);
    assert_eq!(e1.get_bounds(), rect);
    assert_eq!(e2.get_bounds(), Rect::new(pos, size));
    assert_eq!(e3.get_bounds(), rect);
}
