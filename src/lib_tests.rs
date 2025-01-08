use super::*;

#[test]
fn date_is_valid_test() {
    let date1 = Date {
        year: 2025,
        month: 1,
        day: 1,
    }; // Valid
    assert!(date1.is_valid());

    let date2 = Date {
        year: 100,
        month: 2,
        day: 29,
    }; // Valid because of leap year
    assert!(date2.is_valid());

    let date3 = Date {
        year: 101,
        month: 2,
        day: 29,
    }; // Not valid because not a leap year
    assert!(!date3.is_valid());

    let date4 = Date {
        year: 2000,
        month: 6,
        day: 31,
    }; // Not valid
    assert!(!date4.is_valid());

    let date5 = Date {
        year: 2030,
        month: 13,
        day: 1,
    }; // Not valid
    assert!(!date5.is_valid());

    let date6 = Date {
        year: 2030,
        month: 5,
        day: 0,
    }; // Not valid
    assert!(!date6.is_valid());
}

#[test]
fn is_after_test() {
    let date1 = Date {
        year: 1987,
        month: 6,
        day: 29,
    };
    let date2 = Date {
        year: 1987,
        month: 6,
        day: 30,
    };
    let date3 = Date {
        year: 1987,
        month: 7,
        day: 1,
    };
    let date4 = Date {
        year: 1999,
        month: 12,
        day: 31,
    };
    let date5 = Date {
        year: 2000,
        month: 1,
        day: 1,
    };

    assert!(date2.is_after(&date1));
    assert!(date3.is_after(&date2));
    assert!(date5.is_after(&date4));
    assert!(date4.is_after(&date3));
    assert!(!date1.is_after(&date3));
}

#[test]
fn is_day_after_test() {
    let date1 = Date {
        year: 1987,
        month: 6,
        day: 29,
    };
    let date2 = Date {
        year: 1987,
        month: 6,
        day: 30,
    };
    let date3 = Date {
        year: 1987,
        month: 7,
        day: 1,
    };
    let date4 = Date {
        year: 1999,
        month: 12,
        day: 31,
    };
    let date5 = Date {
        year: 2000,
        month: 1,
        day: 1,
    };

    assert!(date2.is_day_after(&date1));
    assert!(date3.is_day_after(&date2));
    assert!(date5.is_day_after(&date4));
    assert!(!date4.is_day_after(&date3));
    assert!(!date1.is_day_after(&date3));
}

#[test]
fn time_is_valid_test() {
    let time1 = Time {
        hours: 0,
        minutes: 20,
    }; // Valid
    assert!(time1.is_valid_time_of_day());

    let time2 = Time {
        hours: 25,
        minutes: 20,
    }; // Not valid
    assert!(!time2.is_valid_time_of_day());

    let time3 = Time {
        hours: 1,
        minutes: 60,
    }; // Not valid
    assert!(!time3.is_valid_time_of_day());
}

#[test]
#[should_panic]
fn time_difference_test() {
    let time1 = Time {
        hours: 1,
        minutes: 20,
    };
    let time2 = Time {
        hours: 5,
        minutes: 10,
    };
    let time3 = Time {
        hours: 1,
        minutes: 15,
    };

    assert_eq!(Time::difference(&time1, &time2).unwrap(),
               Time {
                   hours: 3,
                   minutes: 50,
               });
    Time::difference(&time1, &time3).unwrap(); // Panics (time3 before time1)
}

#[test]
fn time_add_test() {
   let time1 = Time {
        hours: 1,
        minutes: 20,
    };
    let time2 = Time {
        hours: 5,
        minutes: 10,
    };

    assert_eq!(time1 + time2,
               Time {
                   hours: 6,
                   minutes: 30,
               });
}

#[test]
fn write_to_file_test() -> Result<(), Box<dyn Error>> {
    let record = Record {
        /* This test passes if the write is successful. It doesn't check what
         * is written. The output in test.ron should be checked manually.
         */
        
        note: String::from("Writing test functions"),
        date: Date {
            year: 2025,
            month: 1,
            day: 1,
        },
        start_time: Time {
            hours: 17,
            minutes: 0,
        },
        end_time: Time {
            hours: 17,
            minutes: 20,
        },
    };
    let records = vec!(record);
    
    let habit = Habit {
        name: String::from("Testing"),
        records: records,
    };
    let habits = vec!(habit);

    let app_data = AppData {
        version: AppData::CURRENT_VERSION,
        habits: habits,
    };

    app_data.write_to_file("test.ron")
}

#[test]
fn read_from_file_test() {
    /* This test can fail if run concurrently with write_from_file_test.
     * If it fails, try running with --test-threads 1.
     */
    
    let record = Record {
        note: String::from("Writing test functions"),
        date: Date {
            year: 2025,
            month: 1,
            day: 1,
        },
        start_time: Time {
            hours: 17,
            minutes: 0,
        },
        end_time: Time {
            hours: 17,
            minutes: 20,
        },
    };
    let records = vec!(record);
    
    let habit = Habit {
        name: String::from("Testing"),
        records: records,
    };
    let habits = vec!(habit);

    let reference_app_data = AppData {
        version: AppData::CURRENT_VERSION,
        habits: habits,
    };

    let file_app_data = AppData::read_from_file("test.ron").expect(
        "Failed to read data file. Make sure to run tests with 1 thread.\n"
    );
    assert_eq!(reference_app_data, file_app_data);
}
