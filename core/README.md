# Widgets UI toolkit

Prototype user interface library for Rust. It aims to be a simple and efficient UI toolkit
with fresh ideas that does things in a Rusty way instead of the traditional OOP way.

Some of the things featured:

- Non-OOP design: It moves away from the traditional class-inheritance scheme that's used
  in popular GUI toolkits. Things are done with just structs and traits.
- No predefined containers: There are no things like `RelativeLayout` where you create a
  container object and add widgets into it. Any regular struct can be made a container, and
  the layout process is made in a immediate way with just function calls.
- Derivable traits: You can easily create new widgets by deriving most of the required
  traits. It's aimed to be an easy DIY widget toolkit.
- Automatic event dispatch: event propagation in custom widgets is fully handled by the
  library using visitors.
- Decoupled backend: The application code never touches the backend directly, so they can run
  on separate threads/processes, or even switch backend at runtime (not yet implemented).
  The application code receives events and outputs draw commands as data.
- Fully static dispatched: There is no dyn trait used at all (or even supported). Everything is
  done statically using generics and the visitor pattern. If you need variant types, you can
  derive enum dispatch types.

Work in progress. It's still on the early brainstorm phase.

Name is just a placeholder (naming projects is the hardest problem in computer science).
