#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{error::Error, ffi::CString, io, io::prelude::*, str};

use nix::{
    fcntl::{open, OFlag},
    libc::{STDIN_FILENO, STDOUT_FILENO},
    sys::{stat::Mode, wait::{self, waitpid}},
    unistd::{close, dup2, execvp, fork, pipe, write, ForkResult},
};

fn main() {
    // В расте unwrap возвращает значение из пары (значение, ошибка),
    // при этом если была ошибка, то программа упадёт.
    // let x = write(STDOUT_FILENO, b"hi!\n").unwrap();  // TODO: не компилится, но смысл пока понятен.
    // println!("{}", x);

    // Строки должны быть c-style, то есть 0-terminated, а в Расте строки хранят длину.
    // execvp("echo", vec!["echo", "hi"].as_slice());  // does not compile

    // CString::new вернёт ошибку, если строку нельзя сконвертировать: "ec\0ho"
    // let cmd = CString::new("echo").unwrap();
    // let args = vec![CString::new("echo").unwrap(), CString::new("hi").unwrap()];
    // После этой строчки, программа заменится на echo hi, код после этой строчки не будет исполнятся.
    // execvp(cmd.as_c_str(), args.as_slice()).unwrap();

    let fork_result = unsafe { fork().unwrap() };
    print!("Hello, I am... ");  // напечатается дважды

    // Раст не даст потерять варианты енама.
    // match fork_result {
    //     ForkResult::Child => {
    //         println!("Child!");
    //     }
    //     ForkResult::Parent { child } => {
    //         println!("Parent, child_pid = {}!", child);
    //         waitpid(child, None).unwrap();
    //     }
    // }

    // Собираем всё вместе:
    // форкаемся
    // ребёнок запускает execvp,
    // вывод перенаправляется в файл,
    // файл открывается только на запись, создаётся если не существует, и создаётся с MODE / правами 644 (об этом позже)
    // родитель ждёт завершения ребёнка.
    match fork_result {
        ForkResult::Child => {
            println!("Child!");
            let cmd = CString::new("echo").unwrap();
            let args = vec![CString::new("echo").unwrap(), CString::new("hi").unwrap()];

            let fd = open("out.txt", OFlag::O_WRONLY | OFlag::O_CREAT, Mode::from_bits_truncate(0o644)).unwrap();
            dup2(fd, STDOUT_FILENO).unwrap();
            // Закроем оригинальный фд на out.txt, т.к. уже есть копия.
            // Размер таблицы файловых дескрипторов ограничен, хорошая привычка - закрывать ненужные дескрипторы.
            close(fd).unwrap();
            execvp(&cmd, args.as_slice()).unwrap();
        }
        ForkResult::Parent { child }=> {
            println!("Parent, child_pid = {}!", child);
            waitpid(child, None).unwrap();
        }
    }

    // В домашке будет цикл по форкам, дети вызывают execvp, а каждый родитель ждёт своего ребенка.
}
