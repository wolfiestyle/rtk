# Widgets UI toolkit

Prototype user interface library for Rust. It aims to be a simple and efficient UI toolkit
with fresh ideas that does things in a Rusty way instead of the traditional OOP way.

Some of the things featured:

- Trait-based design: It moves away from the traditional class-inheritance scheme that's
  used in popular GUI toolkits. Things are done with just structs and traits.
- No predefined containers: There are no things like `RelativeLayout` where you create a
  container object and add widgets into it. Any regular struct can be made a container.
- Immediate layout: the layout process is made in an immediate way with just function calls.
  All widgets have position and size that is updated in the `update_layout` method.
- Derivable traits: You can easily create new widgets by deriving most of the required
  traits. It's aimed to be an easy DIY widget toolkit.
- Automatic event dispatch: event propagation in custom widgets is fully handled by the
  library using visitors. Just derive `Visitable` and it's done.
- Fully static dispatched: There is no dyn trait used at all. Everything is done statically
  using generics and the visitor pattern. If you need dynamic dispatch, you can derive enum
  dispatch types.
- Pluggable back-ends: It can work with any backend that accepts the `TopLevel` trait.
  Application code has no knowledge of the implementation, and can work with any backend
  without modifications.

Work in progress. It's still on the early brainstorm phase.

Name is just a placeholder (naming projects is the hardest problem in computer science).
