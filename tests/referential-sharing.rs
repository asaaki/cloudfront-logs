use cloudfront_logs::referential;
use std::sync::Arc;

const LINE: &str = "2024-02-29\t12:34:56\tLAX1\t392\t192.0.2.100\tGET\td.example\t/\t200\t-\tagent\t-\t-\tFutureResult\tid\td.example\thttps\t23\t0.001\t192.0.2.7,\\x202001:db8::7\tTLSv1.2\tcipher\tFutureResponse\tHTTP/2.0\t-\t-\t11040\t0.001\tFutureDetail\ttext/html\t78\t-\t-";

fn assert_send_sync<T: Send + Sync>() {}

// These checks exercise the library's owner/dependent lifetime and conversion
// contracts under Arc composition, including allocated fields and thread moves.
macro_rules! sharing_contract {
    ($name:ident, $record:ty) => {
        #[test]
        fn $name() {
            assert_send_sync::<$record>();
            assert_send_sync::<Arc<$record>>();
            let input: Arc<str> = LINE.into();
            let expected = <$record>::try_from(Arc::clone(&input)).unwrap();
            let original = Arc::new(<$record>::try_from(Arc::clone(&input)).unwrap());
            let first = Arc::clone(&original);
            let second = Arc::clone(&first);
            drop(original);
            assert_eq!(first.view(), expected.view());
            assert_eq!(second.as_raw(), LINE);

            // Shared extraction invokes the existing Clone implementation once.
            // It must retain the input allocation and leave other views valid.
            let raw = Arc::unwrap_or_clone(Arc::clone(&first)).into_raw();
            assert!(Arc::ptr_eq(&raw, &input));
            assert_eq!(first.view(), expected.view());
            drop(first);
            let joined = std::thread::spawn(move || {
                assert_eq!(second.as_raw(), LINE);
                second
            })
            .join()
            .unwrap();
            // Unique extraction does not clone the parsed record.
            let unique = Arc::try_unwrap(joined).unwrap();
            assert_eq!(unique, expected);
            assert!(Arc::ptr_eq(&unique.into_raw(), &input));

            for record in [
                <$record>::try_from(LINE).unwrap(),
                <$record>::try_from(LINE.to_owned()).unwrap(),
                <$record>::try_from(Box::<str>::from(LINE)).unwrap(),
            ] {
                assert_eq!(record, expected);
            }

            // No separate owner retains this fresh input allocation.
            let surviving = {
                let original = Arc::new(<$record>::try_from(LINE.to_owned()).unwrap());
                Arc::clone(&original)
            };
            assert_eq!(surviving.view(), expected.view());
            assert_eq!(
                Arc::try_unwrap(surviving).unwrap().into_raw().as_ref(),
                LINE
            );
        }
    };
}

sharing_contract!(raw_validated, referential::ValidatedRawLogline);
sharing_contract!(raw_unvalidated, referential::UnvalidatedRawLogline);
sharing_contract!(simple_validated, referential::ValidatedSimpleLogline);
sharing_contract!(simple_unvalidated, referential::UnvalidatedSimpleLogline);
#[cfg(any(feature = "jiff", feature = "time", feature = "chrono"))]
sharing_contract!(typed_validated, referential::ValidatedTypedLogline);
#[cfg(any(feature = "jiff", feature = "time", feature = "chrono"))]
sharing_contract!(typed_unvalidated, referential::UnvalidatedTypedLogline);
#[cfg(feature = "parquet")]
sharing_contract!(parquet_validated, referential::ValidatedParquetLogline);
#[cfg(feature = "parquet")]
sharing_contract!(parquet_unvalidated, referential::UnvalidatedParquetLogline);

#[test]
fn shared_structured_values_keep_allocated_fields_and_validation_conversion() {
    use cloudfront_logs::EdgeResultType;
    let original = Arc::new(referential::ValidatedSimpleLogline::try_from(LINE).unwrap());
    let forwarded_ptr = original.view().x_forwarded_for.as_ref().unwrap().0.as_ptr();
    let EdgeResultType::Other(result) = &original.view().x_edge_result_type else {
        panic!("fixture must retain its unknown result");
    };
    let result_ptr = result.as_ptr();
    let sibling = Arc::clone(&original);
    drop(original);
    let view = sibling.view();
    assert_eq!(
        view.x_edge_result_type,
        EdgeResultType::Other("FutureResult".into())
    );
    assert_eq!(view.x_forwarded_for.as_ref().unwrap().0.len(), 2);
    assert_eq!(
        view.x_forwarded_for.as_ref().unwrap().0.as_ptr(),
        forwarded_ptr
    );
    let EdgeResultType::Other(result) = &view.x_edge_result_type else {
        panic!("unknown result was lost");
    };
    assert_eq!(result.as_ptr(), result_ptr);
    assert_eq!(view.sc_bytes, 392);
    let validated = Arc::try_unwrap(sibling).unwrap();
    let raw_ptr = validated.as_raw().as_ptr();
    let unvalidated: referential::UnvalidatedSimpleLogline = validated.into();
    assert_eq!(unvalidated.as_raw().as_ptr(), raw_ptr);
    let validated: referential::ValidatedSimpleLogline = unvalidated.into();
    assert_eq!(validated.as_raw().as_ptr(), raw_ptr);
    assert_eq!(
        validated.view().x_forwarded_for.as_ref().unwrap().0.len(),
        2
    );
}

#[test]
fn boxed_input_is_send_and_sync_too() {
    assert_send_sync::<Box<str>>();
    assert_send_sync::<Arc<str>>();
}

#[cfg(any(feature = "jiff", feature = "time", feature = "chrono"))]
#[test]
fn typed_validation_conversions_preserve_shared_values() {
    let original = Arc::new(referential::ValidatedTypedLogline::try_from(LINE).unwrap());
    let sibling = Arc::clone(&original);
    drop(original);
    let expected_datetime = sibling.view().datetime();
    let record = Arc::try_unwrap(sibling).unwrap();
    let raw_ptr = record.as_raw().as_ptr();
    let unvalidated: referential::UnvalidatedTypedLogline = record.into();
    let validated: referential::ValidatedTypedLogline = unvalidated.into();
    assert_eq!(validated.as_raw().as_ptr(), raw_ptr);
    assert_eq!(validated.view().datetime(), expected_datetime);
}
