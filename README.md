Dedupe Contacts
===============

This program is used to find and mark duplicate contacts inside spreadsheets (csv) in the following ways:

1. Searching a single spreadsheet and finding duplicates within.
  1. ex: `dedupe contacts.csv`
2. Searching a spreadsheet for duplicates against a base spreadsheet.
  2. ex: `dedupe base-contacts.csv new-contacts.csv`

In order protect the integrity of the input files, the
program will output the marked results to a new file.

### Spreadsheet columns must be in the following order:

**Last Name, First Name, Company, Phone Number**


### About

This was used as an excuse to learn and practice Rust. I am not actively developing this, however, I would accept
pull requests if someone is interested.

By David Raifaizen
