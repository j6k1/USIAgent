(function() {var implementors = {};
implementors["usiagent"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/event/enum.SystemEventKind.html\" title=\"enum usiagent::event::SystemEventKind\">SystemEventKind</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>",synthetic:false,types:[]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/event/enum.UserEventKind.html\" title=\"enum usiagent::event::UserEventKind\">UserEventKind</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>",synthetic:false,types:[]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/event/enum.SelfMatchEventKind.html\" title=\"enum usiagent::event::SelfMatchEventKind\">SelfMatchEventKind</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>",synthetic:false,types:[]},{text:"impl&lt;'a, T, K, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sys_common/poison/struct.PoisonError.html\" title=\"struct std::sys_common::poison::PoisonError\">PoisonError</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/mutex/struct.MutexGuard.html\" title=\"struct std::sync::mutex::MutexGuard\">MutexGuard</a>&lt;'a, T&gt;&gt;&gt; for <a class=\"enum\" href=\"usiagent/error/enum.EventDispatchError.html\" title=\"enum usiagent::error::EventDispatchError\">EventDispatchError</a>&lt;'a, T, K, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + 'a,<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::EventDispatchError"]},{text:"impl&lt;'a, T, K, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/error/enum.EventHandlerError.html\" title=\"enum usiagent::error::EventHandlerError\">EventHandlerError</a>&lt;K, E&gt;&gt; for <a class=\"enum\" href=\"usiagent/error/enum.EventDispatchError.html\" title=\"enum usiagent::error::EventDispatchError\">EventDispatchError</a>&lt;'a, T, K, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::EventDispatchError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"usiagent/error/struct.DanConvertError.html\" title=\"struct usiagent::error::DanConvertError\">DanConvertError</a>&gt; for <a class=\"enum\" href=\"usiagent/error/enum.ToMoveStringConvertError.html\" title=\"enum usiagent::error::ToMoveStringConvertError\">ToMoveStringConvertError</a>",synthetic:false,types:["usiagent::error::ToMoveStringConvertError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/error/enum.ToMoveStringConvertError.html\" title=\"enum usiagent::error::ToMoveStringConvertError\">ToMoveStringConvertError</a>&gt; for <a class=\"enum\" href=\"usiagent/error/enum.UsiOutputCreateError.html\" title=\"enum usiagent::error::UsiOutputCreateError\">UsiOutputCreateError</a>",synthetic:false,types:["usiagent::error::UsiOutputCreateError"]},{text:"impl&lt;T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/error/enum.UsiOutputCreateError.html\" title=\"enum usiagent::error::UsiOutputCreateError\">UsiOutputCreateError</a>&gt; for <a class=\"enum\" href=\"usiagent/error/enum.EventHandlerError.html\" title=\"enum usiagent::error::EventHandlerError\">EventHandlerError</a>&lt;T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::EventHandlerError"]},{text:"impl&lt;T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;E&gt; for <a class=\"enum\" href=\"usiagent/error/enum.EventHandlerError.html\" title=\"enum usiagent::error::EventHandlerError\">EventHandlerError</a>&lt;T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::EventHandlerError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/error/enum.UsiOutputCreateError.html\" title=\"enum usiagent::error::UsiOutputCreateError\">UsiOutputCreateError</a>&gt; for <a class=\"enum\" href=\"usiagent/error/enum.InfoSendError.html\" title=\"enum usiagent::error::InfoSendError\">InfoSendError</a>",synthetic:false,types:["usiagent::error::InfoSendError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/num/struct.ParseIntError.html\" title=\"struct core::num::ParseIntError\">ParseIntError</a>&gt; for <a class=\"enum\" href=\"usiagent/error/enum.TypeConvertError.html\" title=\"enum usiagent::error::TypeConvertError\">TypeConvertError</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::TypeConvertError"]},{text:"impl&lt;'a, T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sys_common/poison/struct.PoisonError.html\" title=\"struct std::sys_common::poison::PoisonError\">PoisonError</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/mutex/struct.MutexGuard.html\" title=\"struct std::sync::mutex::MutexGuard\">MutexGuard</a>&lt;'a, T&gt;&gt;&gt; for <a class=\"enum\" href=\"usiagent/error/enum.USIAgentRunningError.html\" title=\"enum usiagent::error::USIAgentRunningError\">USIAgentRunningError</a>&lt;'a, T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + 'a,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::USIAgentRunningError"]},{text:"impl&lt;'a, T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/error/enum.USIAgentStartupError.html\" title=\"enum usiagent::error::USIAgentStartupError\">USIAgentStartupError</a>&lt;E&gt;&gt; for <a class=\"enum\" href=\"usiagent/error/enum.USIAgentRunningError.html\" title=\"enum usiagent::error::USIAgentRunningError\">USIAgentRunningError</a>&lt;'a, T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + 'a,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::USIAgentRunningError"]},{text:"impl&lt;E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/error/enum.TypeConvertError.html\" title=\"enum usiagent::error::TypeConvertError\">TypeConvertError</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;&gt; for <a class=\"enum\" href=\"usiagent/error/enum.SelfMatchRunningError.html\" title=\"enum usiagent::error::SelfMatchRunningError\">SelfMatchRunningError</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::SelfMatchRunningError"]},{text:"impl&lt;E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;RecvError&gt; for <a class=\"enum\" href=\"usiagent/error/enum.SelfMatchRunningError.html\" title=\"enum usiagent::error::SelfMatchRunningError\">SelfMatchRunningError</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::SelfMatchRunningError"]},{text:"impl&lt;E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;SendError&lt;<a class=\"enum\" href=\"usiagent/selfmatch/enum.SelfMatchMessage.html\" title=\"enum usiagent::selfmatch::SelfMatchMessage\">SelfMatchMessage</a>&gt;&gt; for <a class=\"enum\" href=\"usiagent/error/enum.SelfMatchRunningError.html\" title=\"enum usiagent::error::SelfMatchRunningError\">SelfMatchRunningError</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::SelfMatchRunningError"]},{text:"impl&lt;E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"usiagent/error/enum.SelfMatchRunningError.html\" title=\"enum usiagent::error::SelfMatchRunningError\">SelfMatchRunningError</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::SelfMatchRunningError"]},{text:"impl&lt;E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/error/enum.KifuWriteError.html\" title=\"enum usiagent::error::KifuWriteError\">KifuWriteError</a>&gt; for <a class=\"enum\" href=\"usiagent/error/enum.SelfMatchRunningError.html\" title=\"enum usiagent::error::SelfMatchRunningError\">SelfMatchRunningError</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::SelfMatchRunningError"]},{text:"impl&lt;E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;E&gt; for <a class=\"enum\" href=\"usiagent/error/enum.SelfMatchRunningError.html\" title=\"enum usiagent::error::SelfMatchRunningError\">SelfMatchRunningError</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"usiagent/error/trait.PlayerError.html\" title=\"trait usiagent::error::PlayerError\">PlayerError</a>,&nbsp;</span>",synthetic:false,types:["usiagent::error::SelfMatchRunningError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/error/enum.ToMoveStringConvertError.html\" title=\"enum usiagent::error::ToMoveStringConvertError\">ToMoveStringConvertError</a>&gt; for <a class=\"enum\" href=\"usiagent/error/enum.SfenStringConvertError.html\" title=\"enum usiagent::error::SfenStringConvertError\">SfenStringConvertError</a>",synthetic:false,types:["usiagent::error::SfenStringConvertError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/error/enum.TypeConvertError.html\" title=\"enum usiagent::error::TypeConvertError\">TypeConvertError</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;&gt; for <a class=\"enum\" href=\"usiagent/error/enum.SfenStringConvertError.html\" title=\"enum usiagent::error::SfenStringConvertError\">SfenStringConvertError</a>",synthetic:false,types:["usiagent::error::SfenStringConvertError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/error/enum.SfenStringConvertError.html\" title=\"enum usiagent::error::SfenStringConvertError\">SfenStringConvertError</a>&gt; for <a class=\"enum\" href=\"usiagent/error/enum.KifuWriteError.html\" title=\"enum usiagent::error::KifuWriteError\">KifuWriteError</a>",synthetic:false,types:["usiagent::error::KifuWriteError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"usiagent/error/enum.KifuWriteError.html\" title=\"enum usiagent::error::KifuWriteError\">KifuWriteError</a>",synthetic:false,types:["usiagent::error::KifuWriteError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a><a class=\"enum\" href=\"usiagent/shogi/enum.Teban.html\" title=\"enum usiagent::shogi::Teban\">Teban</a>, <a class=\"enum\" href=\"usiagent/shogi/enum.MochigomaKind.html\" title=\"enum usiagent::shogi::MochigomaKind\">MochigomaKind</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"enum\" href=\"usiagent/shogi/enum.KomaKind.html\" title=\"enum usiagent::shogi::KomaKind\">KomaKind</a>",synthetic:false,types:["usiagent::shogi::KomaKind"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"usiagent/rule/struct.LegalMoveTo.html\" title=\"struct usiagent::rule::LegalMoveTo\">LegalMoveTo</a>&gt; for <a class=\"struct\" href=\"usiagent/rule/struct.AppliedMoveTo.html\" title=\"struct usiagent::rule::AppliedMoveTo\">AppliedMoveTo</a>",synthetic:false,types:["usiagent::rule::AppliedMoveTo"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"usiagent/rule/struct.LegalMovePut.html\" title=\"struct usiagent::rule::LegalMovePut\">LegalMovePut</a>&gt; for <a class=\"struct\" href=\"usiagent/rule/struct.AppliedMovePut.html\" title=\"struct usiagent::rule::AppliedMovePut\">AppliedMovePut</a>",synthetic:false,types:["usiagent::rule::AppliedMovePut"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/rule/enum.LegalMove.html\" title=\"enum usiagent::rule::LegalMove\">LegalMove</a>&gt; for <a class=\"enum\" href=\"usiagent/rule/enum.AppliedMove.html\" title=\"enum usiagent::rule::AppliedMove\">AppliedMove</a>",synthetic:false,types:["usiagent::rule::AppliedMove"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/shogi/enum.Move.html\" title=\"enum usiagent::shogi::Move\">Move</a>&gt; for <a class=\"enum\" href=\"usiagent/rule/enum.AppliedMove.html\" title=\"enum usiagent::rule::AppliedMove\">AppliedMove</a>",synthetic:false,types:["usiagent::rule::AppliedMove"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/rule/enum.LegalMove.html\" title=\"enum usiagent::rule::LegalMove\">LegalMove</a>&gt; for <a class=\"enum\" href=\"usiagent/shogi/enum.Move.html\" title=\"enum usiagent::shogi::Move\">Move</a>",synthetic:false,types:["usiagent::shogi::Move"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"usiagent/rule/enum.AppliedMove.html\" title=\"enum usiagent::rule::AppliedMove\">AppliedMove</a>&gt; for <a class=\"enum\" href=\"usiagent/shogi/enum.Move.html\" title=\"enum usiagent::shogi::Move\">Move</a>",synthetic:false,types:["usiagent::shogi::Move"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()