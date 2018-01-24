# Merlin
Merlin is a web app providing a CRUD interface for a friend's RPG.

# TODO:
* Render the rest of the references between tables
    * Components per Spell
    * Spells per Component
    * Spells per Category
    * Component Subets and Supersets
    * Category requirements
* Edit content for the rest of the tables
    * Spell
    * Component
    * Category
* Edit references between tables
    * Spell Components
    * Spell Categories
    * Component Subsets
    * Category requirements
* Rename Spells, Categories, Components, and Pages
* Model all data
    * Intervals for Spell duration and casting time (probably switch to seconds)
    * Spell Components
    * Component Subsets
    * Category Requirements
    * Get diesel Associations working
* Cleanups
    * Find a way to unify templates for better reuse
    * Move route handlers into dedicated modules (Pages, Spells, Components, Categories)
* Authentication and user accounts (avoid spambots)
* Tweak schema
    * Replace Intervals with something simpler to model, like integer milliseconds
    * Replace Numeric with something simpler to model (Real?)
    * Replace linking tables with json columns (slow table scans for some data (maybe cached?), but free pg row limit constraints will be a problem soonish)
* Improve aesthetics
    * Better colorscheme
    * Better header, sidebar, navbar, and main content
    * Better forms
    * Render item content formatting
    * Convert current markup from weird old Textile formatting to Markdown
* Build improvements
    * Script to "docker build" with dedicated persistent target directory
    * Configure testing/staging/prod pipeline in heroku