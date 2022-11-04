struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {

    // Gets the area code of the phone number of the person's job, if it exists.
    // その人の市外局番が存在する場合、取得する。
    fn work_phone_area_code(&self) -> Option<u8> {
        // This would need many nested `match` statements without the `?` operator.
        // It would take a lot more code - try writing it yourself and see which
        // is easier.
        // `?`がなければ、多くのネストされた`match`文を必要とするため、より長いコードとなる。
        // 実際に書いて、どちらの方が簡単か確かめてみましょう。
        // self.job?.phone_number?.area_code
        
        match self.job {
            Some(job) => match job.phone_number {
                Some(phone_number) => match phone_number.area_code {
                    Some(area_code) => phone_number.area_code,
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    println!("{:?}", p.work_phone_area_code());
    assert_eq!(p.work_phone_area_code(), Some(61));
}
