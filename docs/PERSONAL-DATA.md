# Personal data

What this repository never holds, where a personal value lives instead, and the two values it allows.

## Values that never enter this repository

1. A person's name, handle, account id or email address.
2. A host name, domain name, home path, device name or network address.
3. A room member's display name or chat platform id.
4. A company name.
5. A line of a real conversation.

None of these goes into code, a code comment, a document, a test fixture or a commit message. The
content holds the same rule in its own way: a personal value in a persona layer or the lore is a
placeholder whose value the deployment fills in.

## Where the value lives instead

1. The deployment keeps every value in its own configuration, outside every repository, and fills each
   placeholder as it places the layers.
2. An example written here uses a neutral name, never a value taken from a person, a room or a machine.
3. A test fixture is written the same way: its names, addresses and ids are invented and neutral.

## The allowed values

The owner's handle `ahas` and address `dev@ahas.io` may stand as written. The personal-data check lets
these two through and catches every other shape.

## The check

A pattern check reads the content, `docs/`, `README.md` and `CLAUDE.md`, and names each line holding
the shape of an email address, a home path, a phone number, a token, a private or public IP address, a
web address, a host name, an account handle or a long numeric id. It searches for shapes, not for a
list of names.
