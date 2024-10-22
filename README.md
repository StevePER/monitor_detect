Rust code that exhibits undefined behaviour. This code uses Windows API calls to gather information
about available monitors. The Windows code is `unsafe`.
- In debug builds, the code runs as expected.
- In rust < 1.78.0, the code runs as expected.
- In release builds in rust >= 1.78.0, the program fails to run as expected with result != TRUE
  even though result and TRUE both have the same value and are the same type.

Example output when working:
```
result=1, TRUE=1
result=1, TRUE=1
Display left 0 right 2560 top 0 bottom 1392
Display left -2560 right 0 top 0 bottom 1392
```

Output with release builds in rust >= 1.78.0:
```
result=1, TRUE=1
Error enumerating monitor information. GetMonitorInfoW failed, result != TRUE
result=1, TRUE=1
Error enumerating monitor information. GetMonitorInfoW failed, result != TRUE
```

*The fix*
To allow rust to track variable usage correctly, we need to change one line in the callback function.
Change this:
`let monitors: &mut Vec<MONITORINFOEXW> = mem::transmute(userdata);`
to this:
`let monitors = &mut *(userdata as *mut Vec<MONITORINFOEXW>);`
