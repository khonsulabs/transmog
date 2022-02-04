(function() {var implementors = {};
implementors["transmog_async"] = [{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, F:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"transmog_async/struct.TransmogReader.html\" title=\"struct transmog_async::TransmogReader\">TransmogReader</a>&lt;R, T, F&gt;","synthetic":false,"types":["transmog_async::reader::TransmogReader"]},{"text":"impl&lt;W:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, D:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, F:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"transmog_async/struct.TransmogWriter.html\" title=\"struct transmog_async::TransmogWriter\">TransmogWriter</a>&lt;W, T, D, F&gt;","synthetic":false,"types":["transmog_async::writer::TransmogWriter"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"transmog_async/struct.AsyncDestination.html\" title=\"struct transmog_async::AsyncDestination\">AsyncDestination</a>","synthetic":false,"types":["transmog_async::writer::AsyncDestination"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"transmog_async/struct.SyncDestination.html\" title=\"struct transmog_async::SyncDestination\">SyncDestination</a>","synthetic":false,"types":["transmog_async::writer::SyncDestination"]},{"text":"impl&lt;TReads:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, TWrites:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, TStream:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, TDestination:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, TFormat:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"transmog_async/struct.TransmogStream.html\" title=\"struct transmog_async::TransmogStream\">TransmogStream</a>&lt;TReads, TWrites, TStream, TDestination, TFormat&gt;","synthetic":false,"types":["transmog_async::TransmogStream"]}];
implementors["transmog_cbor"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"transmog_cbor/enum.Error.html\" title=\"enum transmog_cbor::Error\">Error</a>","synthetic":false,"types":["transmog_cbor::Error"]}];
implementors["transmog_json"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"transmog_json/enum.Error.html\" title=\"enum transmog_json::Error\">Error</a>","synthetic":false,"types":["transmog_json::Error"]}];
implementors["transmog_versions"] = [{"text":"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"transmog_versions/enum.Error.html\" title=\"enum transmog_versions::Error\">Error</a>&lt;E&gt;","synthetic":false,"types":["transmog_versions::Error"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"transmog_versions/struct.UnknownVersion.html\" title=\"struct transmog_versions::UnknownVersion\">UnknownVersion</a>","synthetic":false,"types":["transmog_versions::UnknownVersion"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()