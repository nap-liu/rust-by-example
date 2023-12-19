//!
//! 标准库还提供了很多其他的特性常用的有下面这些
//!
//! - 线程（Threads）
//! - 通道 (Channels)
//! - 文件 (File I/O)
//!
//! 这些扩展方法提供了更多操作系统底层的能力。
//!

///
/// 线程
/// `Rust` 提供了一种机制来可以直接调用系统的线程能力，`Rust` 线程和系统线程是 `1:1` 的关系，
/// 也就是一个 `Rust` 线程等于一个系统的线程。
///
fn threads() {
    use std::thread;

    const NTHREADS: u32 = 10;

    // 创建一个动态数组来保存线程信息。
    let mut children = vec![];

    for i in 0..NTHREADS {
        // 使用标准库 `spawn` 方法来创建线程，并把创建线程的句柄（线程信息）保存在当前线程的变量中。
        // `move` 关键字会转移变量所有权到线程内部。
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // 线程句柄对象上提供了一个方法叫 `join` 来等待线程执行完成，该方法会阻塞当前线程，
        // 直到等待的线程结束退出，当前线程才会继续执行。
        let _ = child.join();
    }
}

///
/// 线程小测验
///
/// `Rust` 可以通过线程非常容易的让处理数据的过程并行，并消除了传统上多线程的一些痛点。
///
/// 标准库提供了非常好用的线程处理能力，组合使用 `Rust` 的所有权和别名规则，则可以自动的防止数据竞争的问题。
///
/// 别名规则 （只允许一个可变引用或者多个不可变引用）自动的帮你避免了在多线程中操作数据不一致的问题。
/// 如果真的需要多个线程同时修改同一个数据的话可以通过标准库提供的 `Mutex` 或 `Channel` 来实现。
///
/// 下面这个例子我们会把一堆数字做一个汇总，这里使用多线程能力并行的处理每一小块数据，每一个线程会单独计算
/// 一小块数据，然后我们会把所有线程计算的结果再进行汇总计算。
///
/// 注意一下，因为我们只是给线程传递了一个引用，`Rust` 明白我们只是传递了一个不可变引用，所以不会产生安全
/// 和数据的竞争问题。
/// 又因为我们传递的引用拥有 `'static` 声明周期，`Rust` 可以知道我们的数据不会在子线程的执行过程中被销毁。
/// （当你需要在多个线程之间分享一个不是 `'statis` 声明周期的数据的时候，可以使用智能指针 `Arc` 来
/// 保证数据在线程的执行过程中不会被销毁。）
///
fn thread_test_case_map_reduce() {
    use std::thread;

    // 这个是我们需要处理的数据，我们会使用线程计算每一行的所有数字的和，然后把每一行的和再进行合并。

    // TODO: 尝试插入一个空格，看看会发生什么
    let data = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    38397966707106094172783238747669219
    52380795257888236525459303330302837
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668";

    // 这里定义一个动态数组来存放线程句柄。
    let mut children = vec![];

    // 这里通过内置方法把上面的字符串按照空白字符分割成字符串的数组。
    for line in data.split_whitespace() {
        // 这里通过标准库的线程方法对每一个字符串进行加和计算，并返回加和的结果值
        children.push(thread::spawn(move || -> u32 {
            // 使用字符串的 `chars` 方法创建一个迭代器，这个迭代器每次都返回一个数字的字符。
            line.chars()
                // 对每一个数字的字符转换成数字
                .map(|f| f.to_digit(10).expect("必须是数字字符串"))
                // 对转换完的数字结果进行加和
                .sum()
        }));
    }

    // 让当前线程等待子线程的计算完成，然后对计算结果再次进行加和计算，最终保存结果到 `sum` 中。
    let sum = children.into_iter().map(|f| f.join().unwrap()).sum::<u32>();

    println!("thread testcase: sum result {}", sum);

    // 实际上对于上面这种处理逻辑是不明智的，因为这里会根据用户输入的数据来创建的线程，而线程是非常宝贵的计算资源，
    // 应该按照一个固定的线程数量来处理用户输入的数据，而不是动态的创建数量不定的线程。
}

///
/// 通道（channels）
///
/// Rust 提供了异步的 `channels` 来让多个线程之间进行数据交换，`channel` 通过 `Sender`，`Receiver` 提供了一个单向的数据交换的能力。
///
fn channels() {
    use std::sync::mpsc;
    use std::thread;

    let NTHREADS = 3i32;

    // 创建一个 `channel`
    let (tx, rx) = mpsc::channel::<i32>();

    let mut children = vec![];

    for id in 0..NTHREADS {
        // 复制一个 `channel` 给每一个线程
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {
            // 通过 `channel` 发送异步数据。
            thread_tx.send(id).unwrap();
        });

        // 保存线程句柄
        children.push(child);
    }

    // 创建一个数组用于保存结果。
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // 通过 `channel` 的接收端接收发送的数据。
        ids.push(rx.recv().unwrap());
    }

    for child in children {
        // 等待所有线程退出
        child.join().expect("oops! the child thread panicked");
    }

    // 这里接收到的结果不保证顺序，因为线程是由操作系统来调度的，所以没办法保证顺序。
    println!("{:?}", ids);
}

///
/// 路径 （Path）
///
/// Path 结构是用于描述底层文件系统的。有两种常用的路径，一种是 `posix::Path` 用于 `UNIX-like` 的系统，
/// 另外一种是 `windows::Path` 是 `Windows` 系统使用的，`Rust` 中会根据当前的系统自动的选择这两种的其中一种。
///
/// `Path` 可以通过 `OsStr` 创建，并且提供了一些方法来获取路径上对应的信息，是一个文件还是文件夹。
///
/// `Path` 是不可变的，可变版本的 `Path` 是 `PathBuf`，这两个的关系和 `str` 和 `String` 的关系非常相似，
/// `PathBuf` 是可变的并且可以取消对原始的 `Path` 的引用关系。
///
/// 注意 `Path` 不会保证路径的内容是 `UTF-8` 格式的，内部使用的是 `OsString` 这个类型来保存数据，因此把 `Path` 转换为
/// `&str` 可能会失败，但是 `Path` 可以通过 `into_os_string` 和 `as_os_str` 转换为 `OsString` 或 `&OsStr`，并且一定会成功。
///
///
fn path_() {
    use std::path::Path;

    // 从 `&'static str` 转换为 `Path`
    let path = Path::new(".");

    // `display` 方法返回一个可以输出的版本。
    let _display = path.display();

    // `join` 方法可以合并两个路径成一个完整的路径，会返回一个 `PathBuf` 的结构。
    let mut new_path = path.join("a").join("b");

    // 使用 `push` 可以在现有的路径上继续添加新的路径。
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name` 方法可以在路径上添加一个文件名。
    new_path.set_file_name("package.tgz");

    // 转换路径到 `&str` 这里可能会失败，因为 `Path` 中可能包含非 `UTF-8` 的内容
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

///
/// 文件 I/O 操作
///
/// File 对象是一个已经打开的文件（内部封装了文件描述符 FD File Descriptor），并且给你提供了
/// 对于底层文件的读写能力。
///
/// 由于文件处理的过程中很多动作可能会失败，所以所有的文件操作都会返回 `io::Result<T>` 类型，也就是
/// `Result<T, io::Error>` 类型。
///
/// 这样所有的操作都会明确的告诉你可能会失败，多亏了这层包装，开发者可以非常明确的看到所有可能失败的操作是什么，
/// 并鼓励积极主动的处理这些问题。
///
fn file_I_O() {
    use std::fs::{self, File};
    // 把常用的方法扩展都引用进来
    use std::io::prelude::*;
    use std::io::{self, BufRead};
    use std::path::Path;

    // 打开并读取文件，只读模式
    {
        let path = Path::new("hello.txt");
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("cloldn't read: {}: {}", display, why),
            Ok(_) => println!("{} contains: \n{}", display, s),
        }
    }

    // 创建文件
    // create 方法会用写文件(write-only)的权限打开指定的文件，如果文件已经存在了则会清空原始文件内容，
    // 文件不存在的话则会创建一个新的文件。
    {
        let LOREM_IPSUM = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";
        let path = Path::new("lorem_ipsum.txt");
        let display = path.display();

        // 只写模式打开文件 返回 `io::Result<File>` 对象。
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // 把 `LOREM_IPSUM` 写入到文件中，返回 `io::Result<()>`。
        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }

        // 还可以使用 `OpenOptions` 这个对象来配置打开文件的权限
    }

    // `read_lines` 很多种场景下都会需要按行读取文件内容，一开始接触文件系统的时候可能会写出下面的方法。
    // 下面实现了三种方法 `read_lines_with_for`，`read_lines_with_iter`，`read_lines`，
    // 这三种方法中前两种在处理大文件的时候都会有很严重的性能问题，因为一次性读取了所有的文件内容，所以会占用很多的内存空间
    // 并且因为所有的内容都被读取到文件中了，所以 `lines` 会处理很大的内容。
    // 而第三种方法 `read_lines` 则不会有这种问题，因为每次读取的时候只会读取一行内容，并且只在需要的时候才会去读取，
    // 如果没有对 `read_lines` 返回的迭代器进行读取操作的话，则文件不会被读取。
    {
        // 最简单直接的读取文件，使用 `for` 循环来处理每一行
        fn read_lines_with_for(filename: &str) -> Vec<String> {
            let mut result = Vec::new();
            // 一次性读取所有的文件内容，然后遍历保存到动态数组中，
            // 这里 `read_to_string` 方法会读取全部的文件内容到内存中，这样的话如果文件非常大，则会造成内存的过度使用。
            for line in fs::read_to_string(filename).unwrap().lines() {
                result.push(line.to_string());
            }
            result
        }

        // 使用迭代器来处理每一行，这两种方法都会造成内存的过度使用。
        fn read_lines_with_iter(filename: &str) -> Vec<String> {
            // 一次性读取所有的文件内容
            fs::read_to_string(filename)
                .unwrap()
                // 按照每一行分割
                .lines()
                // 转化成 `String` 类型
                .map(|f| f.to_string())
                // 把迭代器转换成数组
                .collect()
        }

        // 使用 `std::io::BufReader<File>` 对象来分批次的读取文件，这里使用了 `lines` 方法每次只从文件中读取一行内容。
        fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where
            P: AsRef<Path>, // 这里的类型定义为了保持和 `File::open` 的定义相同
        {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
        }

        // 文件 hosts.txt 必须要存在
        if let Ok(lines) = read_lines("./hosts.txt") {
            // 使用迭代器方法每次只读取一行内容，直到文件全部都被读取完，因为读取操作可能失败，
            // 所以这里使用 `if let` 方法进行解构。
            for line in lines {
                if let Ok(ip) = line {
                    println!("{}", ip);
                }
            }
        }
    }
}

///
/// 子进程
/// 另一个非常常见的场景就是启动一个子进程来处理一下事情，Rust 提供了一个 `process::Command` 对象来专门
/// 提供了这种能力，这个方法会返回一个 `process:Output` 的对象来保存子进程的返回值。
///
fn child_processes() {
    use std::process::Command;

    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}

///
/// 管道（pipes）
///
/// `process::Command` 会返回一个 `process::Child` 的实例对象，该对象表示启动的子进程。
///
/// `process::Child` 对象上提供了 `stdin`，`stdout`，`stderr` 方法来操作系统进程底层的管道
/// 来和子进程进行交互。
///
fn pipes() {
    use std::io::prelude::*;
    use std::process::{Command, Stdio};

    static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";
    // 启动一个 `wc` 的子进程。
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    // 向 `wc` 子进程的输入管道中写入数据，
    // 这里使用了 `unwrap` 获取了 `stdin` 对象并转移了 `stdin` 的所有权。
    // 当 `write_all` 方法执行完以后，内部保留的 `stdin` 对象就会被回收，也就是底层的管道会被自动关闭。
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    // `stdout` 字段是使用  `Option<ChildStdout>` 包装的所以也需要使用 `unwrap` 解包装。
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}

///
/// 有很多时候需要等待子进程的退出，可以使用 `Child::wait` 方法来实现该功能，
/// 这个方法会等待子进程退出并且返回进程的退出状态 `process::ExitStatus`
///
fn wait() {
    use std::process::Command;
    // 这里启动一个子进程 `sleep` 并且传递了一个参数 `5`
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    // `sleep` 进程会等待 `5` 秒钟以后退出，所以这里也会等待 `5` 秒钟。
    let _result = child.wait().unwrap();
    println!("reached end of main");
}

///
/// 文件系统的操作示例
/// Rust 的 std::fs 模块提供了很多方法来操作底层的文件系统
///
fn filesystem_operations() {
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io;
    use std::io::prelude::*;
    use std::os::unix;
    use std::path::Path;

    // 实现一个简单版本的 `cat path` 能力
    fn cat(path: &Path) -> io::Result<String> {
        let mut f = File::open(path)?;
        let mut s = String::new();

        // 这个 match 可以使用下面的代码代替，更简洁一些
        // match f.read_to_string(&mut s) {
        //     Ok(_) => Ok(s),
        //     Err(e) => Err(e),
        // }
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // 实现一个简单版本的 `echo s > path`
    fn echo(s: &str, path: &Path) -> io::Result<()> {
        let mut f = File::create(path)?;

        f.write_all(s.as_bytes())
    }

    // 简单版本的 `touch path` 忽略文件已经存在的错误
    fn touch(path: &Path) -> io::Result<()> {
        match OpenOptions::new().create(true).write(true).open(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    println!("`mkdir a`");
    // 创建文件夹，该方法返回 `io::Result<()>`
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("`echo hello > a/b.txt`");
    // 使用自己实现的能力来写文件，如果出错了的话打印错误
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p a/c/d`");
    // 递归创建文件夹，如果出错打印出错误
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    // 使用自己实现的方法来创建一个文件，创建失败打印错误。
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s ../b.txt a/c/b.txt`");
    // 创建一个软连接，这里只实现 `unix` 的版本
    if cfg!(target_family = "unix") {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    println!("`cat a/c/b.txt`");
    // 使用自己的方法来输出文件。
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls a`");
    // 读取文件夹
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }

    println!("`rm a/c/e.txt`");
    // 删除文件
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`rmdir a/c/d`");
    // 删除一个空目录
    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

///
/// 进程参数
///
/// 标准库提供了 `std::env::args` 用于获取启动进程的参数，该方法返回一个参数的迭代器。
///
fn program_arguments() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    // 读取第一个参数，通常情况下第一个参数都是当前的进程的完整路径。
    println!("My path is {}.", args[0]);

    // 剩下的参数才是启动进程的时候传递的真正参数
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}

///
/// 参数的使用
///
fn arguments_parsing() {
    use std::env;

    fn increase(number: i32) {
        println!("{}", number + 1);
    }

    fn decrease(number: i32) {
        println!("{}", number - 1);
    }

    fn help() {
        println!(
            "usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one."
        );
    }

    let args: Vec<String> = env::args().collect();

    match args.len() {
        // 没有参数
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
        }
        // 一个参数
        2 => match args[1].parse() {
            Ok(42) => println!("This is the answer!"),
            _ => println!("This is not the answer."),
        },
        // 两个参数，一个是方法，一个是具体数字
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // 格式化数字
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                }
            };
            // 选择使用的方法
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                }
            }
        }
        // 都不匹配的话则全部都走一个兜底的方法
        _ => {
            // 显示帮助消息
            help();
        }
    }
}

///
/// 外部函数引用（FFI Foreign Function Interface）
///
/// Rust 提供了引用外部 `C` 库函数的能力（FFI），外部函数必须定义在 `extern` 块内，
/// 然后通过属性宏 `#[link(name = "m")]` 声明，下面定义的函数名在哪个库内。
///
fn foreign_function_interface() {
    use std::fmt;

    // Minimal implementation of single precision complex numbers
    // 单精度复数的最小实现
    #[repr(C)] // 定义数据的对齐方式使用 `C` 的方式
    #[derive(Clone, Copy)]
    struct Complex {
        re: f32,
        im: f32,
    }

    impl fmt::Debug for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.im < 0. {
                write!(f, "{}-{}i", self.re, -self.im)
            } else {
                write!(f, "{}+{}i", self.re, self.im)
            }
        }
    }

    // 这个表示这里定义的所有方法都是 `libm` 这个库中的方法。
    #[link(name = "m")]
    extern "C" {
        // 这个方法是计算单精度复数的平方根，调用的是 `libm` 库中的方法。
        fn csqrtf(z: Complex) -> Complex;
        fn ccosf(z: Complex) -> Complex;
    }

    // 因为外部方法默认都是不安全的 （unsafe），必须使用 `unsafe {}` 来包裹外部的不安全方法调用，
    // 经过包装以后对外提供一个安全方法，可以直接调用，不再需要 `unsafe {}` 来包裹了。
    fn cos(z: Complex) -> Complex {
        unsafe { ccosf(z) }
    }
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // 直接调用外部方法 需要使用 `unsafe {}` 来包裹
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    // calling safe API wrapped around unsafe operation
    // 这里调用我们自己封装的安全方法，这样的话就不需要 `unsafe {}` 包裹了
    println!("cos({:?}) = {:?}", z, cos(z));
}

fn main() {
    // 线程操作
    threads();
    // 线程小测验
    thread_test_case_map_reduce();
    // 通道
    channels();
    // 文件路径
    path_();
    // 文件IO
    file_I_O();
    // 子进程
    child_processes();
    // 子进程和管道
    pipes();
    // 等待子进程
    wait();
    // 操作文件系统
    filesystem_operations();
    // 进程参数
    program_arguments();
    // 进程参数的使用
    arguments_parsing();
    // 外部函数使用 `FFI`
    foreign_function_interface();
}
