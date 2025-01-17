# Ellie Changelog

All notable changes to Ellie will be documented in this file. This project adheres to the standards set out by
the [Semantic Versioning][semver] specification.

## [0.7.0 Carbon][0.7.0] - Not yet released

This is a *major* release of Ellie, culminating over a year of work to completely rework Ellie and her internal components
to allow her to work asynchronously instead of synchronously. This rewrite also required me to rework my `lastfm-rs`
crate to also use async / await, but that didn't take much effort or time. This is such a big release, that I decided to
make the jump to version 0.7.0 from 0.5.0. No 1.0 yet, as the bot doesn't have anywhere close to the amount of features I
plan on implementing for Ellie in the long-term.

**Personal note**: This rewrite is taking me longer than I initially would've hoped. In late September 2020, I got a job,
and it took nearly all my focus away from working on Ellie. This job however ends on December 17th, at which point I hope
by then this rewrite will be entirely complete.

**UPDATE**: I have returned to my previous job since August 2021, so work on this project has sadly stalled. However I
intend on allocating more free time towards working on Ellie.

Main headlining features include:

- Asynchronous support via `async/await`.
- Several reworks to various commands.
- A shift from the previous SQLite database implementation that used `rusqlite` to a brand new PostgreSQL 13-based database
  using the `sqlx` crate.
- A migration over to a new TOML-based configuration system from the prior implementation that used the `.dev` configuration
  format / system.
- A brand new voice system (finally!), using Lavalink and the brand new `songbird` crate. (temporarily removed)

Please read on below to learn about all of the changes included in this release. It is a big one, so only read this at your
leisure.

### Architectural Changes

- Foundation
  - Ellie has been rewritten to take advantage of asynchronous serenity, which means Ellie should perform a bit
    faster.
  - Ellie has been moved to a PostgreSQL-based database using the `sqlx` crate instead of SQLite and the `rusqlite` crate,
    meaning database requests are now asynchronous as well.

### New Features

- Configuration System
  - Moved to a TOML-based configuration system. This allows more flexibility and control than the previous `.env`
    system allowed as well as a cleaner overall configuration format.
- Commands
  - Fun
    - Added `ascii` command. This allows converting text to display in an ASCII font. You can use custom fonts
      with the command as well by providing a supported font as an argument. You can find more information about
      this in the command's help documentation.
    - Added `printerfacts` command. This retrieves a random "fact" about printers. This command uses Christine
      Dodrill's [Printer Facts](https://printerfacts.cetacean.club) API.
  - Info
    - Added `about` command. This displays various statistics about Ellie, such as version and various statistics
      about the users, guilds, shards, and channels she can see.
  - Moderation
    - Added `slowmode` command. This allows server moderators / administrators to control a channel's slowmode
      setting.
  - Search
    - Added a set of commands that integrate with The Movie Database. This includes commands such as `show`, which
      allows retrieving information about a specified television show, `movie` which gets information on a specified
      movie, `collection` which retrieves information about a specified film collection, and `cast`, which retrieves
      details about the cast and crew of a specific movie or television series.

### Improvements

- Core
  - Added handling when a user only provide's Ellie's prefix, giving help if the user wants to learn more about her
    available functionality.
  - Moved the various parsing utilities to their own file, to reduce the amount of clutter in the other utility files.
- Commands
  - `channel`:
    - Improved the parsing of channels. It now recognizes strings, channel mentions, and generic search strings.
      This does **NOT** support trying to search for channels by separating with a space, due to Discord channel
      names not supporting spaces.
    - Added the channel's category name, if available.
  - `changelog`: Improved the embed's title to be more informative. **Note**: Not yet relevant, as this command has
    not yet been re-implemented in the async rewrite.
  - `guild`:
    - Roles are now sorted by their position in the Server Settings' Roles list.
    - Added support for displaying the guild's System Channel used for sending Discord system messages.
  - `lastfm`:
    - Added ability to view a user's top artists.
    - Moved to embed fields, improving the internal code handling of the command, as well as improving the actual
      design of how the command is structured.
  - `spotify`:
    - All Commands:
      - Moved to using embed fields.
    - `album`
      - Moved away from the embed author instead of using the specific embed fields for title, url, and
        thumbnail.
      - Ellie now displays the total length of the album specified.
  - `profile`: Implemented the ability to set gender, location, and pronouns.
- Database
  - Improved the way fields in the `profile` table are retrieved. The functions that were present in the database utility
    file were entirely removed and replaced with a single `get_profile_fields` function, that accesses the respective column
    via a `field` function parameter. This reduced a lot of boilerplate that was present, and allows me to have one function
    to retrieve all fields.

- Event Handler
  - Added handling of the Guild Create Discord event. This allows us to set a server's default prefix when Ellie
    joins a guild / server that she does not yet recognize. If Ellie already recognizes a server, the insertion
    query is ignored.

### Bug Fixes

- Dispatcher
  - Fixed a bug where if more arguments were supplied to a command supporting `min_args` and `max_args` than allowed,
    Ellie would log a Dispatcher error on top of the messages already being sent to the channel that initialized the
    given command. This is now fixed, and Ellie properly responds when more or less arguments that are required are
    fed to a command.

### Removed Functionality

- Commands
  - Extra
    - The `weather` command has been removed, due to the impending shutdown of the Dark Sky API at the end of 2022,
      due to Apple having acquired Dark Sky in 2019. The command may be re-added eventually, if I can find a suitable
      weather API to replace Dark Sky, but for now, the command has been removed.

  - Info
    - `profile`: Removed all social fields except for the Last.fm field. I didn't entirely understand why I bothered
      implementing those fields to begin with, but they have been completely removed. New profile fields may be added
      again eventually, but for now I am happy with there only being fields for name, pronouns, etc.
  - Utilities
    - `prefix`:
      - Removed the `get` subcommand in favor of just showing the set prefix when a user just runs the `prefix`
        command. This cleans up a bit of code, and improves general functionality of the command instead of just
        showing an "Invalid Subcommand" message when no subcommand is invoked. **NOTE**: Not implemented.
    - Removed `version`, in favor of `about`. `about` doesn't display as much information about Ellie's version
      information that `version` did, however the `about` command provides more meaningful information.

## [0.5.0 Boron][0.5.0] - February 9, 2020

Fairly significant update, adding various commands, and vastly improving most other commands. This also includes
various bug fixes, but they are squeezed together as part of the various improvements to commands.

### New Features

- General
  - Added a GitHub Actions pipeline that runs on every commit that runs `cargo check` and several Rust formatting
    utilities like `clippy` and `rustfmt` to help make sure that the project is adhering to normal Rust code styling.
  - Added this changelog!
  - Updated several dependencies.
- Commands
  - Added `changelog` command. Polls the GitHub GraphQL API for recent commits to Ellie's master branch.
  - Added `source` command. Sends a message containing a URL to the bot's source code.
  - `github`
    - Added `repository` command. Retrieves information about a GitHub repository.
    - Added `user` command. Retrieves information about a GitHub user.
  - `info`
    - Added `role` command. Retrieves information on a specified guild role.
    - Added `channel` command. Retrieves information about a specified guild channel
      or category.
  - `profile`
    - Added Twitch, PlayStation and Xbox user ID support. PlayStation and Xbox user IDs however do not currently link
      to anything as Microsoft and Sony do not allow profiles to be publicly displayed, and as such this information will
      be purely for lookup purposes.
  - `reddit`
    - Added `user` command. Gets information about a specified Reddit user.
    - Added `subreddit` command. Gets information about a specified subreddit on Reddit.
  - `spotify`
    - Added `credits` command. Shows the credits for a specified track.
    - Added `artist` command. Retrieves information about a Spotify artist.
    - Added `status` command. Retrieves the message author's, or a specified users' Spotify status.
  - `user`
    - Added rich presence support for Spotify and Visual Studio Code.
  - voice:
    - Added a basic subset of voice commands. This includes `play`, `leave`, and `join`.They're not that fully featured due
      to various issues with serenity's voice implementation, however there are plans on the serenity developer's roadmap
      to improve / revamp serenity's voice subsystem, so hopefully once it's revamped, this command set will get an overhaul.

### Improvements

- Commands
  - `guild`
    - Improved the way `str`'s are handled, reducing a lot of the `to_owned()` cloning that was previously done.
    - Added support for viewing the guild's multi-factor authentication level.
    - Added support for displaying "x users, x bots" for Members.
    - Dropped the filtering of channel categories for the channel count and instead provided an additional filtering
      type for categories that shows up next to the total amount of channels.
  - `lastfm`
    - Improved the way Last.fm errors are handled, and more errors are now handled, such as `OperationFailed`.
    - Removed unnecessary logging. Specifically, Ellie no longer warns when no track attributes could be found for a track,
      as that is expected behavior for Last.fm tracks that aren't currently being scrobbled.
    - Improved the way album artwork is retrieved, moving entirely to the Spotify Web API for album artwork retrieval instead
      of going through both the Last.fm and Spotify Web API endpoints.
  - `profile`
    - Improved the way users are parsed, and Discord user IDs are now supported when looking up a profile.
  - `spotify`
    - Cleaned up the general structure of the Spotify command system.
  - `user`
    - Roles are now @'d. They now show up in their beautuiful colored form.
    - Added a Profile field. This just @'s a user's handle silently and allows you to view the user's profile just by clicking
      the name. **Note**: This does not work on mobile due to the way Discord handles the embed system on mobile devices.
    - Completely revamped how Online Statuses are handled. This entails various enhancements such as being able to view a
      user's client status, and see any and all activities the user is doing with the exception of being able to view Custom
      Statuses, which are not supported due to how they would look with any other activities. At some point, I may look into
      adding support for Custom Statuses, depending on if I can find a solution I like with regards to this.
    - Just like with `profile`, user parsing has been improved.

[semver]: http://semver.org
[0.5.0]: https://github.com/KamranMackey/Ellie/compare/v0.4.2..v0.5.0
[0.7.0]: https://github.com/KamranMackey/Ellie/compare/v0.5.0..main
