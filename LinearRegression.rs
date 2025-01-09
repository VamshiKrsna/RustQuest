use std::f64;

// Linear Regression Model
struct LinearRegression {
    slope: f64,
    intercept: f64,
}

impl LinearRegression {
    // constructor
    fn new() -> Self {
        LinearRegression {
            slope: 0.0,
            intercept: 0.0,
        }
    }

    // mean of a vector
    fn mean(&self, x:&[f64]) -> f64 {
        x.iter().sum::<f64>() / x.len() as f64
    }

    // fit the model
    fn fit(&mut self, x:&[f64], y:&[f64]) {
        // mean of x and y 
        let mean_x = self.mean(x);
        let mean_y = self.mean(y);

        // calculate slope (m) and intercept (b)
        let mut num = 0.0; // numerator
        for i in 0..x.len() {
            let diff_x = x[i] - mean_x; // differences between xi and mean of x
            let diff_y = y[i] - mean_y; // differences between yi and mean of y
            num += diff_x * diff_y; // sum of product of differences
        }

        let mut denom = 0.0; // denominator
        for i in 0..x.len(){
            let diff_x = x[i] - mean_x; // differences between xi and mean of x
            denom += diff_x.powi(2); // sum of square of differences
        }

        self.slope = num / denom; // slope
        self.intercept = mean_y - self.slope * mean_x; // intercept
    }

    // predict y for given x
    fn predict(&self, x:f64) -> f64 {
        self.slope * x + self.intercept // y = mx + b
    }
    
}

fn main(){
    // example data
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 3.0, 4.0, 5.0, 6.0];

    // create model
    let mut model = LinearRegression::new();

    // fit the model
    model.fit(&x, &y);

    // predict y for x = 6
    let x_new = 6.0;
    let y_new = model.predict(x_new);
    println!("Predicted y for x = {}: {:.4}", x_new, y_new);
}