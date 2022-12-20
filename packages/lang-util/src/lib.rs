#[macro_export]
macro_rules! retry {
  ($times:expr, $run:expr) => {{
    let n: usize = $times;
    assert!(n != 0, "cannot call retry! with n == 0");

    let mut i: usize = 0;

    #[allow(unused_labels)]
    'retry: loop {
      i += 1;
      match $run {
        Ok(v) => break Ok(v),
        Err(e) => {
          if i == n {
            break Err(e);
          } else {
            continue;
          }
        }
      }
    }
  }};
}

#[cfg(test)]
mod test {
  #[test]
  fn compile_sync() {
    let _r = retry!(5, {
      if true {
        Err(())
      } else {
        Ok(())
      }
    });
  }

  #[test_util::async_test]
  async fn compile_async() {
    let _r = retry!(5, {
      let b = async { true }.await;
      if true {
        Ok(b)
      } else {
        Err(())
      }
    });
  }

  #[test_util::async_test]
  async fn compile_early_return_async() {
    let _r = retry!(5, {
      async {
        if true {
          return Ok(true);
        } else if false {
          return Err(false);
        }

        Err(true)
      }
      .await
    });
  }

  #[test_util::async_test]
  async fn compile_question_return_async() {
    let _r = retry!(5, {
      async {
        let r = Result::<(), ()>::Ok(());
        r?;
        Result::<(), ()>::Ok(())
      }
      .await
    });
  }
}
