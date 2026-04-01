mod numerical_duration {
    use rstest::rstest;
    use time::ext::NumericalDuration;
    use time::Duration;

    #[rstest]
    #[case(5.nanoseconds(), Duration::nanoseconds(5))]
    #[case(5.microseconds(), Duration::microseconds(5))]
    #[case(5.milliseconds(), Duration::milliseconds(5))]
    #[case(5.seconds(), Duration::seconds(5))]
    #[case(5.minutes(), Duration::minutes(5))]
    #[case(5.hours(), Duration::hours(5))]
    #[case(5.days(), Duration::days(5))]
    #[case(5.weeks(), Duration::weeks(5))]
    fn unsigned(#[case] ext_trait: Duration, #[case] expected: Duration) {
        assert_eq!(ext_trait, expected);
    }

    #[rstest]
    #[case((-5).nanoseconds(), Duration::nanoseconds(-5))]
    #[case((-5).microseconds(), Duration::microseconds(-5))]
    #[case((-5).milliseconds(), Duration::milliseconds(-5))]
    #[case((-5).seconds(), Duration::seconds(-5))]
    #[case((-5).minutes(), Duration::minutes(-5))]
    #[case((-5).hours(), Duration::hours(-5))]
    #[case((-5).days(), Duration::days(-5))]
    #[case((-5).weeks(), Duration::weeks(-5))]
    fn signed(#[case] ext_trait: Duration, #[case] expected: Duration) {
        assert_eq!(ext_trait, expected);
    }

    #[rstest]
    #[case::truncate_not_round(1.9.nanoseconds(), Duration::nanoseconds(1))]
    #[case(1.0.nanoseconds(), Duration::nanoseconds(1))]
    #[case(1.0.microseconds(), Duration::microseconds(1))]
    #[case(1.0.milliseconds(), Duration::milliseconds(1))]
    #[case(1.0.seconds(), Duration::seconds(1))]
    #[case(1.0.minutes(), Duration::minutes(1))]
    #[case(1.0.hours(), Duration::hours(1))]
    #[case(1.0.days(), Duration::days(1))]
    #[case(1.0.weeks(), Duration::weeks(1))]
    #[case(1.5.nanoseconds(), Duration::nanoseconds(1))]
    #[case(1.5.microseconds(), Duration::nanoseconds(1_500))]
    #[case(1.5.milliseconds(), Duration::microseconds(1_500))]
    #[case(1.5.seconds(), Duration::milliseconds(1_500))]
    #[case(1.5.minutes(), Duration::seconds(90))]
    #[case(1.5.hours(), Duration::minutes(90))]
    #[case(1.5.days(), Duration::hours(36))]
    #[case(1.5.weeks(), Duration::hours(252))]
    fn float(#[case] ext_trait: Duration, #[case] expected: Duration) {
        assert_eq!(ext_trait, expected);
    }

    #[rstest]
    #[case(2.seconds() + 500.milliseconds(), 2_500.milliseconds())]
    #[case(2.seconds() - 500.milliseconds(), 1_500.milliseconds())]
    fn arithmetic(#[case] arithmetic_: Duration, #[case] expected: Duration) {
        assert_eq!(arithmetic_, expected);
    }
}

mod numerical_std_duration {
    use std::time::Duration as StdDuration;

    use rstest::rstest;
    use time::ext::NumericalStdDuration;

    #[rstest]
    #[case(5.std_nanoseconds(), StdDuration::from_nanos(5))]
    #[case(5.std_microseconds(), StdDuration::from_micros(5))]
    #[case(5.std_milliseconds(), StdDuration::from_millis(5))]
    #[case(5.std_seconds(), StdDuration::from_secs(5))]
    #[case(5.std_minutes(), StdDuration::from_secs(5 * 60))]
    #[case(5.std_hours(), StdDuration::from_secs(5 * 3_600))]
    #[case(5.std_days(), StdDuration::from_secs(5 * 86_400))]
    #[case(5.std_weeks(), StdDuration::from_secs(5 * 604_800))]
    fn unsigned(#[case] ext_trait: StdDuration, #[case] expected: StdDuration) {
        assert_eq!(ext_trait, expected);
    }

    #[rstest]
    #[case::truncate_not_round(1.9.std_nanoseconds(), StdDuration::from_nanos(1))]
    #[case(1.0.std_nanoseconds(), StdDuration::from_nanos(1))]
    #[case(1.0.std_microseconds(), StdDuration::from_micros(1))]
    #[case(1.0.std_milliseconds(), StdDuration::from_millis(1))]
    #[case(1.0.std_seconds(), StdDuration::from_secs(1))]
    #[case(1.0.std_minutes(), StdDuration::from_secs(60))]
    #[case(1.0.std_hours(), StdDuration::from_secs(3_600))]
    #[case(1.0.std_days(), StdDuration::from_secs(86_400))]
    #[case(1.0.std_weeks(), StdDuration::from_secs(604_800))]
    #[case(1.5.std_nanoseconds(), StdDuration::from_nanos(1))]
    #[case(1.5.std_microseconds(), StdDuration::from_nanos(1_500))]
    #[case(1.5.std_milliseconds(), StdDuration::from_micros(1_500))]
    #[case(1.5.std_seconds(), StdDuration::from_millis(1_500))]
    #[case(1.5.std_minutes(), StdDuration::from_secs(90))]
    #[case(1.5.std_hours(), StdDuration::from_secs(90 * 60))]
    #[case(1.5.std_days(), StdDuration::from_secs(36 * 3_600))]
    #[case(1.5.std_weeks(), StdDuration::from_secs(252 * 3_600))]
    fn float(#[case] ext_trait: StdDuration, #[case] expected: StdDuration) {
        assert_eq!(ext_trait, expected);
    }

    #[rstest]
    #[case(2.std_seconds() + 500.std_milliseconds(), 2_500.std_milliseconds())]
    #[case(2.std_seconds() - 500.std_milliseconds(), 1_500.std_milliseconds())]
    fn arithmetic(#[case] arithmetic_: StdDuration, #[case] expected: StdDuration) {
        assert_eq!(arithmetic_, expected);
    }
}
