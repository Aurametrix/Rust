# Rust
One of the most loved by [developers](http://stackoverflow.com/insights/survey/2016#technology-most-loved-dreaded-and-wanted) languages, but not the most popular. Rust is a systems programming language - more like C++ and Go, and less like Node and Ruby. It is designed for better memory safety than C++ while maintaining performance

[Official site](https://www.rust-lang.org/en-US/) and
[Github](https://github.com/rust-lang/rust) 

Python has its "eggs" and Ruby has its "gems". Clojure uses the same boring "jars" as other JVM languages. Rust chose to call its format "crates".



Who is using Rust: https://www.rust-lang.org/en-US/friends.html
[Learn Rust](https://news.ycombinator.com/item?id=24527219)

[comments on](https://news.ycombinator.com/item?id=24294960) [Rust - 1.46](https://blog.rust-lang.org/2020/08/27/Rust-1.46.0.html)
[1.48](https://blog.rust-lang.org/2020/11/19/Rust-1.48.html)

[Learning Rust in 2020](https://github.com/pretzelhammer/rust-blog/blob/master/posts/learning-rust-in-2020.md)

[Speeding up Rust compiler](https://blog.mozilla.org/nnethercote/2020/09/08/how-to-speed-up-the-rust-compiler-one-last-time/)

[Safety in Systems Programing](https://reberhardt.com/blog/2020/10/05/designing-a-new-class-at-stanford-safety-in-systems-programming.html); 
[code for the course](https://github.com/reberhardt7/cs110l-spr-2020-starter-code)

#### Examples of Projects built with Rust:
* [Servo, the new browser engine being developed by Mozilla](https://github.com/servo/servo)
* [Maidsafe, a company that tries to create an encrypted, completely decentralized "successor" to the internet](http://maidsafe.net/)
* [Piston, a modular open source game engine](http://www.piston.rs/)
* [RG3D](https://github.com/mrDIMAS/rg3d) - 3D game engine & scene editor
* [Zinc, an experimental RTOS for ARM](http://zinc.rs/)
* [wtftw, a tiling window manager](https://github.com/Kintaro/wtftw)
* [Redox: A Rust Operating System](https://www.redox-os.org); Github [code](https://github.com/redox-os/redox)
* [Sandspiel game](https://sandspiel.club); Github [code:](https://github.com/maxbittker/sandspiel) 
* [Bevy](https://bevyengine.org/news/introducing-bevy/) - data-driven game engine
* [Rust for Clojurists](https://gist.github.com/oakes/4af1023b6c5162c6f8f0)
+ [Cryptographic hash function](https://github.com/BLAKE3-team/BLAKE3)
+ [Terminal-based habit tracker](https://github.com/NerdyPepper/dijo)

##### Misc
[Comparing performance](https://bitbucket.org/blog/why-rust)
[Rust Analyser]https://(rust-analyzer.github.io/blog/2020/04/20/first-release.html)
[Eust-GPU](https://github.com/EmbarkStudios/rust-gpu/releases/tag/v0.1)
[Your guide to programming ARM Cortex-M microcontrollers with Rust](https://github.com/japaric/copper), [YN](https://news.ycombinator.com/item?id=14071282)

[svd2rust](https://docs.rs/svd2rust/)  - generates register-level APIs from SVD files, an XML format provided by most ARM microcontroller vendors; 

[real-time communication](https://blog.tonari.no/why-we-love-rust)

[Assembly with Rust](https://lfn3.net/2020/08/03/a-gentle-intro-to-assembly-with-rust/): [playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=9500bb2bc3f638a4dd89e81fecafac0e)

[cortex_m](https://docs.rs/cortex-m/) - provides APIs for the core ARM peripherals in every Cortex-M MCU;
[f3](https://docs.rs/f3/) - High level APIs for the peripherals and external sensors on the STM32 F3 Discovery board.

[Busy Box](https://github.com/samuela/rustybox)

[NES emulator](https://github.com/spieglt/nestur)

[Tensorbase](https://tensorbase.io/)

[JQL](https://crates.io/crates/jql) - JSON query language built with RUST

[Rust script](https://rust-script.org/)

add $STANDALONE_NDK/bin to $PATH.

    [target.arm-linux-androideabi]
    ar = "arm-linux-androideabi-ar"
    linker = "arm-linux-androideabi-gcc"

[Gutenberg](https://github.com/Keats/gutenberg) -  static site generator with everything built-in  --[hn](https://news.ycombinator.com/item?id=15507538) 

+ [Bioinformatics Library](https://github.com/10XGenomics/rust-bio)
+ [Game Development](http://iolivia.me/posts/24-hours-of-rust-game-dev/)
+ [Yew: Rust framework for making React-like client web apps](https://github.com/DenisKolodin/yew)
+ [Single File Web App with React and Rust](https://anderspitman.net/2018/04/04/static-react-rust-webapp/)
+ [Deploy on AWS](https://stackoverflow.com/questions/41250087/how-to-deploy-a-react-node-express-application-on-aws)
+ [Rust for Rubyists](https://matthias-endler.de/2017/rust-for-rubyists/)
* [A syntax diagram generator](https://lukaslueg.github.io/macro_railroad_wasm_demo/)
+ [Crepe](https://crates.io/crates/crepe) - library for declarative logic programs in Rust, with a Datalog-like syntax
+ [AutoCXX](https://github.com/google/autocxx) - call C++ from Rust
+ [Command-line library](https://github.com/rust-shell-script/rust_cmd_lib)
+ [GameLisp ](https://gamelisp.rs/) - scripting language for Rust game development; [Tetris](https://gamelisp.rs/playground/#quadris)
