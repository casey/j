// todo:
// - create issue for sighup handling: can we detect when we receive sighup and are the only recipient?
// - fix and test on windows
// - tests:
//   - just terminates immediately on receipt of fatal signal if no child process is running
//   - just reports correct error if child process is terminated with fatal signal and indicates failure
//   - just still terminates if child process is terminated with fatal signal and reports success
//   - just prints info receipt of SIGINFO
//   - contexts: recipe line, recipe script, --command, backtick (any others?)
// - get rid of old interrupt tests
