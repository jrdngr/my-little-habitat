use yew::services::ConsoleService;

pub type Point<T> = (T, T);

pub fn log<T: AsRef<str>>(message: T) {
    ConsoleService::new().log(message.as_ref());
}

pub fn line_between((x1, y1): Point<i32>, (x2, y2): Point<i32>) -> Vec<Point<i32>> {
    if x2 == x1 {
        let min = y1.min(y2);
        let max = y1.max(y2);
        return (min..max).into_iter().map(|y| (x1, y)).collect();
    }

    let m = (y2 - y1) as f64 / (x2 - x1) as f64;
    let b = y1 as f64 - (m * x1 as f64);

    let f = |x| (m * x as f64 + b) as i32;

    let min = x1.min(x2);
    let max = x1.max(x2);

    (min..max).into_iter().map(|x| (x, f(x))).collect()
}
