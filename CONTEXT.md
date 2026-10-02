# Dashboard UI

Shared vocabulary for the visual building blocks of the dashboard window, so layout and styling changes can be described precisely.

## Layout

**Board**:
The whole dashboard window, filled edge to edge with Columns.
_Avoid_: page, screen, app

**Column**:
One of the three vertical stacks of Widgets on the Board.
_Avoid_: lane

**Drag Strip**:
The thin invisible area along the top edge of the Board that moves the window when dragged.
_Avoid_: title bar, handle

## Widget

**Widget**:
A white rounded card showing one data source (RAM, CPU, Spotify, Service Health, Sentry, Docker, Jira). The Spotify Widget is the exception: it has album art as background and no white card.
_Avoid_: tile, card, panel, box

**Widget Title**:
The large teal heading in the top left of a Widget.
_Avoid_: header, tile title

**Header Info**:
Small gray text aligned right on the same line as the Widget Title, summarising the Widget (last checked time, issue count, Docker resource usage).
_Avoid_: subheading, subtitle, info

**Status Message**:
Gray italic text that replaces a Widget's content while it is loading, empty, or failed.
_Avoid_: placeholder, empty state, error text

**Section**:
A part of a Widget separated by a thin divider line and labelled with a Section Label, such as "Top Processes".
_Avoid_: block, area

**Section Label**:
The small gray uppercase label that starts a Section.
_Avoid_: subheading, caption

## Items

**Item**:
A light gray block with an Accent Bar on its left that represents one entity in a list (a container, a service, a Sentry issue, a Jira ticket).
_Avoid_: section, tile, card, row, entry

**Compact Item**:
An Item whose Item Title shares the first Item Line with a right aligned value and is cut off when too long. Used for services and containers.
_Avoid_: small item, short item

**Text Item**:
An Item whose Item Title is long free text that gets the full width and wraps over several lines, followed by further Item Lines. Used for Sentry issues and Jira tickets.
_Avoid_: long item, wrapping item

**Accent Bar**:
The thick colored stripe on the left edge of an Item. Teal means normal, red means something is wrong.
_Avoid_: border, green bar, stripe

**Item Title**:
The bold dark text at the top left of an Item, naming the entity.
_Avoid_: name, heading

**Item Line**:
One horizontal line of text inside an Item, usually with a left part and a right aligned part.
_Avoid_: row

**Group**:
A set of Items under a shared Group Heading, laid out in several side by side columns. Used for the containers of one Docker Compose project.
_Avoid_: section, compose card, cluster

**Group Heading**:
The bold dark label above a Group, naming what its Items share (the compose project folder).
_Avoid_: compose name label, subheading

## Values and indicators

**Badge**:
A small rounded label with a tinted background, holding a short value such as UP/DOWN, a Jira status, or a Sentry event count.
_Avoid_: pill, chip, tag, code badge

**Icon Button**:
A small borderless button showing an icon, sometimes with a short label, that turns teal while its option is active. Used for Filters in Header Info.
_Avoid_: toolbar button, toggle

**Filter**:
An Icon Button that narrows the Items shown in a Widget, such as only my tickets or a single ticket status.
_Avoid_: toggle, view option

**Usage Bar**:
A horizontal bar whose filled length shows a percentage, such as RAM usage or a single CPU core.
_Avoid_: progress bar, meter

**Usage Color**:
A text color that shifts from green through yellow to red as a percentage rises, used for Docker CPU and RAM in Header Info.
_Avoid_: gradient, heat color

**Primary**:
The teal accent color used for Widget Titles, Accent Bars, Usage Bars, and highlighted numbers.
_Avoid_: green, brand color
