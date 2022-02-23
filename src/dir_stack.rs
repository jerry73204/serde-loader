use crate::common::*;

thread_local! {
    static DIR_STACK: RefCell<Vec<PathBuf>> = RefCell::new(vec![]);
}

pub fn try_rebase_path<'a>(path: impl Into<Cow<'a, Path>>) -> Cow<'a, Path> {
    let path = path.into();

    if path.is_absolute() {
        path
    } else {
        DIR_STACK.with(|stack| {
            let stack = stack.borrow();
            match stack.last() {
                Some(dir) => {
                    let path = path.as_ref();
                    dir.join(path).into()
                }
                None => path,
            }
        })
    }
}

pub fn with_rebased_dir<'a, P, F, R>(dir: P, f: F) -> R
where
    P: Into<Cow<'a, Path>>,
    F: FnOnce() -> R,
{
    let dir = dir.into().into_owned();

    DIR_STACK.with(|stack| {
        stack.borrow_mut().push(dir);
    });

    let result = f();

    DIR_STACK.with(|stack| {
        stack
            .borrow_mut()
            .pop()
            .expect("please report bug: unexpected empty directory stack");
    });

    result
}
