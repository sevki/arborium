//// Lustre is a framework for rendering Web applications and components using
//// Gleam. This module contains the core API for constructing and communicating
//// with Lustre applications. If you're new to Lustre or frontend development in
//// general, make sure you check out the [examples](https://github.com/lustre-labs/lustre/tree/main/examples)
//// or the [quickstart guide](./guide/01-quickstart.html) to get up to speed!
////
//// Lustre currently has three kinds of application:
////
//// 1. A client-side single-page application: think Elm or React or Vue. These
////    are applications that run in the client's browser and are responsible for
////    rendering the entire page.
////
//// 2. A client-side component: an encapsulated Lustre application that can be
////    rendered inside another Lustre application as a Web Component. Communication
////    happens via attributes and event listeners, like any other HTML element.
////
//// 3. A server component. These are applications that run anywhere Gleam runs
////    and communicate with any number of connected clients by sending them
////    patches to apply to their DOM.
////
////    There are two pieces to a server component: the main server component
////    runtime that contains your application logic, and a client-side runtime
////    that listens for patches over a WebSocket and applies them to the DOM.
////
////    The server component runtime can run anywhere Gleam does, but the
////    client-side runtime must be run in a browser. To use it, either render the
////    [provided script element](./lustre/server_component.html#script) or serve
////    the pre-bundled scripts found in Lustre's `priv/` directory directly.
////
//// No matter where a Lustre application runs, it will always follow the same
//// Model-View-Update architecture. Popularised by Elm (where it is known as The
//// Elm Architecture), this pattern has since made its way into many other
//// languages and frameworks and has proven to be a robust and reliable way to
//// build complex user interfaces.
////
//// There are three main building blocks to the Model-View-Update architecture:
////
//// - A `Model` that represents your application's state and an `init` function
////   to create it.
////
//// - A `Msg` type that represents all the different ways the outside world can
////   communicate with your application and an `update` function that modifies
////   your model in response to those messages.
////
//// - A `view` function that renders your model to HTML, represented as an
////   `Element`.
////
//// To see how those pieces fit together, here's a little diagram:
////
//// ```text
////                                          +--------+
////                                          |        |
////                                          | update |
////                                          |        |
////                                          +--------+
////                                            ^    |
////                                            |    |
////                                        Msg |    | #(Model, Effect(Msg))
////                                            |    |
////                                            |    v
//// +------+                         +------------------------+
//// |      |  #(Model, Effect(Msg))  |                        |
//// | init |------------------------>|     Lustre Runtime     |
//// |      |                         |                        |
//// +------+                         +------------------------+
////                                            ^    |
////                                            |    |
////                                        Msg |    | Model
////                                            |    |
////                                            |    v
////                                          +--------+
////                                          |        |
////                                          |  view  |
////                                          |        |
////                                          +--------+
//// ```
////
//// The `Effect` type here encompasses things like HTTP requests and other kinds
//// of communication with the "outside world". You can read more about effects
//// and their purpose in the [`effect`](./effect.html) module.
////
//// For many kinds of apps, you can take these three building blocks and put
//// together a Lustre application capable of running *anywhere*. Because of that,
//// we like to describe Lustre as a **universal framework**.
////
//// ## Guides
////
//// A number of guides have been written to teach you how to use Lustre to build
//// different kinds of applications. If you're just getting started with Lustre
//// or frontend development, we recommend reading through them in order:
////
//// - [`01-quickstart`](./guide/01-quickstart.html)
//// - [`02-state-management`](./guide/02-state-management.html)
//// - [`03-side-effects`](./guide/03-side-effects.html)
//// - [`04-spa-deployments`](./guide/04-spa-deployments.html)
//// - [`05-server-side-rendering`](./guide/05-server-side-rendering.html)
//// - [`06-full-stack-applications`](./guide/06-full-stack-applications.html)
//// - [`07-full-stack-deployments`](./guide/07-full-stack-deployments.html)
//// - [`08-components`](./guide/08-components.html)
//// - [`09-server-components`](./guide/09-server-components.html)
