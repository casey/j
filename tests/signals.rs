// todo:
// - probably first is forward sigterm to child
// - document in readme

// - situations:
//   - kill w/sigterm, sighup, sigint, sigquit
//   - process cooperates / progress ignores signals
//   - ctrl-c, close terminal
//   - local, remote without controlling terminal, remote with controlling terminal, remote in interactive shell
//   - recipe line, recipe script, --command, backtick
//
// - INT, HUP, TERM, QUIT:
//   - if no children, exit immediately
//   - if children, wait for them to finish, exit after they finish
// - QUIT: exit immediately?
// - TERM: forward to children
// - INFO: print info
// - get rid of or adapt interrupt tests
//
// - how to test?
//   - run just
//   - send signal
//   - waits for child
//   - child returns
//
// - tests:
//   - child returns failure after signal
//     - just returns signal number
//   - child returns success after signal
//     - just returns signal number
//     - does not continue
//   - TERM
//     - signal forwarded to child
//   - INFO prints info
