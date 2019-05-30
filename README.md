## `jra`

A CLI to run JIRA queries with clickable links.

```
$ jra ls
 JIRA                           QUERY_NAME  QUERY 
 https://issues.jenkins-ci.org  myreported  project = JENKINS and creator = currentUser() 
$ jra q myreported
 LINK           SUMMARY                                               STATUS    REPORTER         ASSIGNEE     PRIORITY 
 JENKINS-56004  Overall/Administrator required to view s3 artifacts?  Resolved  Peter Salvatore  Jesse Glick  Critical 
```

### Status

Works for me (on x86_64 Linux), but not actively developed.

### Getting Started

1.  Install rust stable from https://rustup.rs
2.  `$ cargo build --release`
3.  Add `./target/release/jra` to your PATH.
4.  Construct a configuration file `$HOME/.config/jra.json` with contents like:
    
        {
            "jenkins": {
                "host": "https://issues.jenkins-ci.org",
                "user": "<username>",
                "pass": "<password>",
                "queries": {
                    "myreported": "project = JENKINS and creator = currentUser()"
                }
            }
        }
