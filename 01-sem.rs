fn main() {
    // В расте unwrap возвращает значение из пары (значение, ошибка),
    // при этом если была ошибка, то программа упадёт.
    let x = write(STDOUT_FILENO, b"hi!\n").unwrap();
    println("{}", x);

    // Строки должны быть c-style, то есть 0-terminated, а в Расте строки хранят длину.
    // execvp("echo", vec!["echo", "hi"].as_slice());  // does not compile

    // CString::new вернёт ошибку, если строку нельзя сконвертировать: "ec\0ho"
    let cmd = CString::new("echo").unwrap();
    let args = vec![CString::new("echo").unwrap(), CString::new("hi").unwrap()];
    execvp(cmd, args.as_slice());
    
    let fork_result = unsafe { fork().unwrap() };
    println!("Hello");  // напечатается дважды
    
    // Раст не даст потерять варианты енама.
    match fork_result {
        ForkResult::Child => {
            println!("Child!");
        }
        ForkResult::Parent => {
            println!("Parent, child_pid = {}!", child);
            waitpid(child, None).unwrap();
        }
    }
    
    // Собираем всё вместе:
    // ребёнок запускает execvp,
    // вывод перенаправляется в файл,
    // файл открывается только на запись, создаётся если не существует, и создаётся с MODE / правами 644 (об этом позже)
    // родитель ждёт завершения ребёнка.
    match fork_result {
        ForkResult::Child => {
            println!("Child!");
            let cmd = CString::new("echo").unwrap();
            let args = vec![CString::new("echo").unwrap(), CString::new("hi").unwrap()];
            
            let fd = open("out.txt", OFlag::OWRONLY | OFlag::O_CREAT, Mode::from_bits_truncate(0o644));
            dup2(fd, STDOUT_FILENO).unwrap();
            // Закроем оригинальный фд на out.txt, т.к. уже есть копия.
            // Размер таблицы файловых дескрипторов ограничен, хорошая привычка - закрывать ненужные дескрипторы.
            close(fd).unwrap();
            execvp(&cmd, args.as_slice()).unwrap();
        }
        ForkResult::Parent => {
            println!("Parent, child_pid = {}!", child);
            waitpid(child, None).unwrap();
        }
    }
    
    // В домашке будет цикл по форкам, дети вызывают execvp, а каждый родитель ждёт своего ребенка.
}
