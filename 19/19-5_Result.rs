mod checked {
    // Mathematical "errors" we want to catch
    // 捕捉対象としたい、数学的な「エラー」
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // This operation would `fail`, instead let's return the reason of
            // the failure wrapped in `Err`
            // 分母が0なので、このオペレーションは普通に行えば失敗する。
            // 代わりに`Err`でラップされた失敗の理由を返そう。
            Err(MathError::DivisionByZero)
        } else {
            // This operation is valid, return the result wrapped in `Ok`
            // このオペレーションは問題がないので、結果を`Ok`でラップして返そう。
            let result = x / y;
            println!("div={}", result);
            Ok(result)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            let result = x.sqrt();
            println!("sqrt={}", result);
            Ok(result)
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            let result = x.ln();
            println!("ln={}", result);
            Ok(result)
        }
    }
}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) -> f64 {
    // This is a three level match pyramid!
    // 3段階の`match`ピラミッド！
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    // Will this fail?
    // これは失敗するだろうか？
    //println!("{}", op(1.0, 10.0));
    println!("op={}", op(100.0, 10.0));
}