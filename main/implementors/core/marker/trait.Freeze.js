(function() {var implementors = {};
implementors["transmog_async"] = [{"text":"impl&lt;R, T, F&gt; Freeze for <a class=\"struct\" href=\"transmog_async/struct.TransmogReader.html\" title=\"struct transmog_async::TransmogReader\">TransmogReader</a>&lt;R, T, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Freeze,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Freeze,&nbsp;</span>","synthetic":true,"types":["transmog_async::reader::TransmogReader"]},{"text":"impl&lt;W, T, D, F&gt; Freeze for <a class=\"struct\" href=\"transmog_async/struct.TransmogWriter.html\" title=\"struct transmog_async::TransmogWriter\">TransmogWriter</a>&lt;W, T, D, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Freeze,<br>&nbsp;&nbsp;&nbsp;&nbsp;W: Freeze,&nbsp;</span>","synthetic":true,"types":["transmog_async::writer::TransmogWriter"]},{"text":"impl Freeze for <a class=\"struct\" href=\"transmog_async/struct.AsyncDestination.html\" title=\"struct transmog_async::AsyncDestination\">AsyncDestination</a>","synthetic":true,"types":["transmog_async::writer::AsyncDestination"]},{"text":"impl Freeze for <a class=\"struct\" href=\"transmog_async/struct.SyncDestination.html\" title=\"struct transmog_async::SyncDestination\">SyncDestination</a>","synthetic":true,"types":["transmog_async::writer::SyncDestination"]},{"text":"impl&lt;TReads, TWrites, TStream, TFormat&gt; Freeze for <a class=\"struct\" href=\"transmog_async/struct.Builder.html\" title=\"struct transmog_async::Builder\">Builder</a>&lt;TReads, TWrites, TStream, TFormat&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TFormat: Freeze,<br>&nbsp;&nbsp;&nbsp;&nbsp;TStream: Freeze,&nbsp;</span>","synthetic":true,"types":["transmog_async::Builder"]},{"text":"impl&lt;TReads, TWrites, TStream, TDestination, TFormat&gt; Freeze for <a class=\"struct\" href=\"transmog_async/struct.TransmogStream.html\" title=\"struct transmog_async::TransmogStream\">TransmogStream</a>&lt;TReads, TWrites, TStream, TDestination, TFormat&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TFormat: Freeze,<br>&nbsp;&nbsp;&nbsp;&nbsp;TStream: Freeze,&nbsp;</span>","synthetic":true,"types":["transmog_async::TransmogStream"]}];
implementors["transmog_bincode"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"transmog_bincode/struct.Bincode.html\" title=\"struct transmog_bincode::Bincode\">Bincode</a>","synthetic":true,"types":["transmog_bincode::Bincode"]}];
implementors["transmog_cbor"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"transmog_cbor/struct.Cbor.html\" title=\"struct transmog_cbor::Cbor\">Cbor</a>","synthetic":true,"types":["transmog_cbor::Cbor"]},{"text":"impl Freeze for <a class=\"enum\" href=\"transmog_cbor/enum.Error.html\" title=\"enum transmog_cbor::Error\">Error</a>","synthetic":true,"types":["transmog_cbor::Error"]}];
implementors["transmog_pot"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"transmog_pot/struct.Pot.html\" title=\"struct transmog_pot::Pot\">Pot</a>","synthetic":true,"types":["transmog_pot::Pot"]}];
implementors["transmog_versions"] = [{"text":"impl&lt;E&gt; Freeze for <a class=\"enum\" href=\"transmog_versions/enum.Error.html\" title=\"enum transmog_versions::Error\">Error</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Freeze,&nbsp;</span>","synthetic":true,"types":["transmog_versions::Error"]},{"text":"impl Freeze for <a class=\"struct\" href=\"transmog_versions/struct.UnknownVersion.html\" title=\"struct transmog_versions::UnknownVersion\">UnknownVersion</a>","synthetic":true,"types":["transmog_versions::UnknownVersion"]}];
implementors["xtask"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"enum\" href=\"xtask/enum.Config.html\" title=\"enum xtask::Config\">Config</a>","synthetic":true,"types":["xtask::Config"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()