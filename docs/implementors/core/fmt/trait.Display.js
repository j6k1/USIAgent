(function() {var implementors = {};
implementors["usiagent"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/event/enum.MovedKind.html\" title=\"enum usiagent::event::MovedKind\">MovedKind</a>","synthetic":false,"types":["usiagent::event::MovedKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/event/enum.Moved.html\" title=\"enum usiagent::event::Moved\">Moved</a>","synthetic":false,"types":["usiagent::event::Moved"]},{"text":"impl&lt;K, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.EventHandlerError.html\" title=\"enum usiagent::error::EventHandlerError\">EventHandlerError</a>&lt;K, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,&nbsp;</span>","synthetic":false,"types":["usiagent::error::EventHandlerError"]},{"text":"impl&lt;'a, T, K, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.EventDispatchError.html\" title=\"enum usiagent::error::EventDispatchError\">EventDispatchError</a>&lt;'a, T, K, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,&nbsp;</span>","synthetic":false,"types":["usiagent::error::EventDispatchError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"struct\" href=\"usiagent/error/struct.InvalidStateError.html\" title=\"struct usiagent::error::InvalidStateError\">InvalidStateError</a>","synthetic":false,"types":["usiagent::error::InvalidStateError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"struct\" href=\"usiagent/error/struct.DanConvertError.html\" title=\"struct usiagent::error::DanConvertError\">DanConvertError</a>","synthetic":false,"types":["usiagent::error::DanConvertError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.ToMoveStringConvertError.html\" title=\"enum usiagent::error::ToMoveStringConvertError\">ToMoveStringConvertError</a>","synthetic":false,"types":["usiagent::error::ToMoveStringConvertError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.UsiOutputCreateError.html\" title=\"enum usiagent::error::UsiOutputCreateError\">UsiOutputCreateError</a>","synthetic":false,"types":["usiagent::error::UsiOutputCreateError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.InfoSendError.html\" title=\"enum usiagent::error::InfoSendError\">InfoSendError</a>","synthetic":false,"types":["usiagent::error::InfoSendError"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.TypeConvertError.html\" title=\"enum usiagent::error::TypeConvertError\">TypeConvertError</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a>,&nbsp;</span>","synthetic":false,"types":["usiagent::error::TypeConvertError"]},{"text":"impl&lt;E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.USIAgentStartupError.html\" title=\"enum usiagent::error::USIAgentStartupError\">USIAgentStartupError</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>","synthetic":false,"types":["usiagent::error::USIAgentStartupError"]},{"text":"impl&lt;'a, T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.USIAgentRunningError.html\" title=\"enum usiagent::error::USIAgentRunningError\">USIAgentRunningError</a>&lt;'a, T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>","synthetic":false,"types":["usiagent::error::USIAgentRunningError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.ShogiError.html\" title=\"enum usiagent::error::ShogiError\">ShogiError</a>","synthetic":false,"types":["usiagent::error::ShogiError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.UsiProtocolError.html\" title=\"enum usiagent::error::UsiProtocolError\">UsiProtocolError</a>","synthetic":false,"types":["usiagent::error::UsiProtocolError"]},{"text":"impl&lt;E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.SelfMatchRunningError.html\" title=\"enum usiagent::error::SelfMatchRunningError\">SelfMatchRunningError</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>","synthetic":false,"types":["usiagent::error::SelfMatchRunningError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.SfenStringConvertError.html\" title=\"enum usiagent::error::SfenStringConvertError\">SfenStringConvertError</a>","synthetic":false,"types":["usiagent::error::SfenStringConvertError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"enum\" href=\"usiagent/error/enum.KifuWriteError.html\" title=\"enum usiagent::error::KifuWriteError\">KifuWriteError</a>","synthetic":false,"types":["usiagent::error::KifuWriteError"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()