///
/// 1)
///  What memory model Rust has?:
///     - one heap, one stack for every thread
/// Is it single-threaded or multiple-threaded?:
///     -
/// Is it synchronous or asynchronous?:
///     -
///
/// 2) What runtime Rust has?:
///  - Runtime is time of program executing
///   Does it use a GC (garbage collector)?:
///   - no it doesn't. Rust would know when the variable gets out of scope or its lifetime ends
///          at compile time and thus insert the corresponding  instructions to free the memory.
///   But for some cases it has, like atomic reference counting. It's allocate variavle on the heap and
///  make memory free when last reference was droped.
///
///  3) Statically typed is a programming language characteristic in which variable types are explicitly
///     declared and determined in compile time. This give rust sefety benefits(like
///      we will not have any surpises at runtime), and we shouldn't determine  it at runtime so it will be faster.
///
///  4) What are generics and parametric polymorphism?: Which problems do they solve?
///     - this is a possibility to get rid of code repeating by writing code
///     without knowledges about concrete type.
///
///   5) What are traits? How are they used?
///     How do they compare to interfaces?
///     What are an auto trait and a blanket impl?
///      What is a marker trait? :
///    -    Traits determine common behavior for structs, helps to write generic funcions
///         and install some borders for it.
///    -  Interface and trait are have a lot of common stuff. Intarface can have const variables, trait doesn'
///    - Trait can have aliases
///    - Marker trait is a blanket trait, it show that some struct is can be used in some way.
///     (can by safty send between treads)
///    - Auto traits is marker traits, every struct is this trait if it not have variable that is not this trait.
///  6)
///     What are static and dynamic dispatches?
///      - static dispatch is when we create generic func and we do know what concrate type will be in that place
///    so compiler use monomorphization to handle it(create new funcion for every type under the hood)
///     and replace call of generic func with concrate. This has a  higher performance.
///     - dynamic  dispathc is binding with concrate type in runtime with trait object &Trait or Box<Trait>
///      compile don't create new func for every struct but determine type in runtime with is slowly
///      We could have trait object by casts and coersion
///
///        cast: let cast = ref_to_t as &Foo;
///         coersion: let coerce: &Foo = ref_to_t;
///         fn also_coerce(_unused: &Foo) {}
///         also_coerce(ref_to_t);
///
///         dynamic dispatch save in table where saves pointer to this object and methods that calls.
///             The pointer but not trait object becouse we need know size of object for
///         moving it about on the stack and allocating (and deallocating) space on the heap to store it.
///          with pointer we need to know only the size of the pointer.
///     Which should I use, and when?
///   dinamic dispatch we need to use when we don't know what concrete type we will have.
///   For example it could be determine by user or request.
///   It other cases better to use static dispath becouse it faster.
///
///   7)  What is a crate and what is a module in Rust?:
/// How do they differ? How are the used?
///    A crate is synonymous with a ‘library’ or ‘package’ in other languages
///    every crate has root module and can be You can then define a tree of sub-modules under that root module.
///      Modules allow you to partition your code within the crate itself.
///
///   8)  What are move semantics?:
///      - object can be moved by captered variables by value .
///       in func params in closure on in other variables.
///     What are borrowing rules?:
///      - many shered reference  or one exclusive
///
///         What is the benefit of using them?: sefety,
///     we know that over object will not be changed by somebody  else.
///   9) What is immutability? What is the benefit of using it?
///   by dafault rust variables are immutable it give a advantage of the safety and easy concurrency
///
///  10) What is immutability? :
///     - all variables in rust are immutable by default.
/// What is the benefit of using it?:
///   - sefety in cuncurency
///   - reduce amount of developer errors.
///     we must not to keep track if variables may be changed, if u didn't make it mut.
/// 11) What is cloning? :
///
///     A common trait for the ability to explicitly duplicate an object.
///  What is copying?  :
///    -    Types whose values can be duplicated simply by copying bits.
///         copying is used implicitly. Type should be Clone to be Copy because Clone is super-trait
///       and should do it like this ( fn clone (&self)->Self{
///                                     *self } )
/// How do they compare?:
///     -  Copies happen implicitly, for example as part of an assignment y = x. The behavior of Copy is not overloadable; it is always a simple bit-wise copy.
///     Cloning is an explicit action, x.clone(). The implementation of Clone can provide
///     any type-specific behavior necessary to duplicate values safely. For example,
///     the implementation of Clone for String needs to copy the pointed-to string buffer in the heap.
///
///   12)   What is RAII?:
///             Resource allocation (or acquisition) is done during object creation (specifically initialization),
///          by the constructor, while resource deallocation (release) is done during object destruction
///         (specifically finalization), by the destructor.
///      destructor:
///         is a method which is invoked mechanically just before the memory of the object is released.
///         It can happen when its lifetime is bound to scope and the execution leaves the scope,
///         when it is embedded in another object whose lifetime ends, or when it was allocated dynamically
///             and is released explicitly.
///         Its main purpose is to free the resources (memory allocations, open files or sockets,
///         database connections, resource locks, etc.
///      How is it implemented in Rust?:
///     When object went out of scope it was freed, or can be droped by explicit call 'drop' call
///       with is empy fn under the hood.
///
///       Destructor in rust is a Drop trait with is should be impliment only if u need to add some
///      operation like connection close or whatever.
///      What is the benefit of using it? :
///       - The developer need not to think about memory freed by himself.
///         this is reduce amount of errors and memory leeks
///      - The system with GC should spend a lot of resourses for make memory free,
///        some of them need to stop execution for this operation.
///
///  13)  What is an iterator?:
///       - and interface of dealing with collection elements.
///        - Include a lot of usefull methods for it.
///         You also can implement iterator,
///          you should implement only next method, other have basic implementation, but may be changed.
///  What is a collection? :
///       - group of similar data object. Collection has underlying data structure
/// like Veckor, linked list, hash map or whatewer.
///
///  How do they differ?      How are they used? :
///           Collection implements Iterator for handling inner objects .
///  14) What are macros?:
///       - Macros enable you to write code that writes other code in compile time.
///     Which problems do they solve?:
///      -  could be used for implementing template code( marker trait or traits with default realization)\
///          that's reduce cod.
///      -  extension of languages possabilites.
///        'string builder example' that add type and name of variables
///      - optimization by compile time execution.
///   similar to a match expression that operates on the Rust code you provide as arguments.
///   15) What is the difference between declarative and procedural macro?
/// declarative:
///   Something similar with match
///  It uses the code you provide to generate code that replaces the macro invocation
///     procedural : Is more like a funcion . It use some code like an input(TokenStream ), operate with this code and than
///    produce some code like an output.
///    The three kinds of procedural macros are custom derive, attribute-like, and function-like
///   derive macro:
///
///  attribute-like: Attribute-like macros enable you to create a
/// custom attribute that attaches itself to an item and allows manipulation of that item
/// example: async_trait macro
///
///   funcion-like:
///          Function-like macros are similar to declarative macros in that they’re invoked
///         with the macro invocation operator ! and look like function calls.
///          They operate on the code that is inside the parentheses.
///
///  16)  How code is tested in Rust?:
///          unit test: test some conrate module and can test private funcions
///          could be declared under the code in current mod.
///
///             integration test: shoulb be in tests folder in the same level with src .
///              there u can use only public structure and methods like clients do.
///               the need to imitate work of programm
///
///            docs test:Documentation comments are written in markdown and support code blocks in them.
///             Rust takes care about correctness,
///                so these code blocks are compiled and used as documentation tests.
///
/// 17) Why Rust has &str and String types? :
///        Becouse &str can be used like
///         - String is the dynamic heap string type
///          we need use it if we want to own string and change it.
///
///         - str: is an immutable1 sequence of UTF-8 bytes of dynamic length
///             someweres in memory.
///             tipicaly we have only reference to it.
///         it holds a pointer to some bites and lenght, becouse we need to know size at compile time.
///           so we need to use it when we need only to read str.
///
///  18) What are lifetimes?:
///          - show how much link to some object will not be droped
///     Which problems do they solve?:
///      prevent from indefinite  behavior.
///       if we will have the pointer to some memory, that was dropped.
///       never know what will happen next.
///    Which benefits do they give? :
///     - give us possability to do not allocate new variables but use already allocated without
///       hasitating about that they was dropped
///
///  19) Is Rust OOP language?:
///         - rust has abstactions - traits
///          -has incapsulation  hide realization + (agreagates data and methods for that data in struct)
///          - doesn't have struct inheritance
///         - it has object and objects has funcion Is it possible to use SOLID/GRASP? Does it have an inheritance?
///           - has polimormism (generics)
///     Solid:
///     single responsability: 1 struct - 1 responsability, everything that need for implementation are
///     incapsulated in it.
///         open-closed: programs open to be extend but closed to be modified
///         Liskov subtitution:
///              fun that use base type, may use subtype withougt knowlages about it
///             segregation interface:
///                     client should not be forced to used methods thats he need not(better to have
///                 a lot small traits than one huge)
///                  dependency inversion:
///                    program should depends on abstractons.
///                 grasp:
///                 - https://ru.wikipedia.org/wiki/GRASP
///
///
///
///
///
///
///
///
///