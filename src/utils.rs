use yew::services::ConsoleService;

pub fn log<T: AsRef<str>>(message: T) {
    ConsoleService::new().log(message.as_ref());
}
