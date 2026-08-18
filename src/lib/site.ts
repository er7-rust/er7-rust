/**
 * Site-wide constants: navigation, external links, and the crate family.
 *
 * Everything here is the single place to change a URL or a version. Page
 * components import from here rather than hard-coding links, so a rename
 * is one edit.
 */

/** Where the crate lives. */
export const links = {
  repository: 'https://github.com/er7-rust/er7-rust',
  organization: 'https://github.com/er7-rust',
  crate: 'https://crates.io/crates/er7',
  docs: 'https://docs.rs/er7/',
  email: 'mailto:joel@joelparkerhenderson.com'
} as const;

/** The crate version this site documents. */
export const version = '0.1.1';

export type NavLink = { href: string; label: string };

/** Main navigation, in reading order. */
export const navLinks: NavLink[] = [
  { href: '/', label: 'Home' },
  { href: '/format/', label: 'ER7 format' },
  { href: '/paths/', label: 'Paths' },
  { href: '/escapes/', label: 'Escapes' },
  { href: '/cli/', label: 'Command line' },
  { href: '/api/', label: 'API' },
  { href: '/er7-redact/', label: 'er7-redact' },
  { href: '/serde-er7/', label: 'serde-er7' },
  { href: '/ecosystem/', label: 'Ecosystem' },
  { href: '/about/', label: 'About' }
];

export type Crate = {
  name: string;
  description: string;
  repository: string;
  crate: string;
  docs: string;
  layer: string;
};

/** The five crates in the family, from the bottom of the stack up. */
export const crates: Crate[] = [
  {
    name: 'er7',
    description:
      'The ER7 encoding: parse, query, edit, and write HL7 v2 messages, with a byte-for-byte round trip and no dependencies.',
    repository: 'https://github.com/er7-rust/er7-rust',
    crate: 'https://crates.io/crates/er7',
    docs: 'https://docs.rs/er7/',
    layer: 'Encoding'
  },
  {
    name: 'er7-redact',
    description:
      'Removes patient detail from a message without moving its shape: a policy of HL7 paths and actions, stable pseudonyms, and a report of every position it changed.',
    repository: 'https://github.com/er7-rust/er7-redact',
    crate: 'https://crates.io/crates/er7-redact',
    docs: 'https://docs.rs/er7-redact/',
    layer: 'Tool'
  },
  {
    name: 'serde-er7',
    description:
      'Gives every er7 type a Serialize and Deserialize implementation, so a message tree can flow through JSON, YAML, or any other Serde data format.',
    repository: 'https://github.com/er7-rust/serde-er7',
    crate: 'https://crates.io/crates/serde-er7',
    docs: 'https://docs.rs/serde-er7/',
    layer: 'Tool'
  },
  {
    name: 'hl7-2-5-to-xml',
    description:
      'Converts HL7 v2.5 messages to the official v2.xml representation, using the v2.5 data-type tables and message-structure grammars.',
    repository: 'https://github.com/joelparkerhenderson/hl7-2-5-to-xml-using-rust',
    crate: 'https://crates.io/crates/hl7-2-5-to-xml',
    docs: 'https://docs.rs/hl7-2-5-to-xml/',
    layer: 'HL7 v2.5 dictionary'
  },
  {
    name: 'hl7-2-5-to-json',
    description:
      'The JSON sibling: the same v2.5 tables and grammars, rendered as idiomatic JSON instead of XML.',
    repository: 'https://github.com/joelparkerhenderson/hl7-2-5-to-json-using-rust',
    crate: 'https://crates.io/crates/hl7-2-5-to-json',
    docs: 'https://docs.rs/hl7-2-5-to-json/',
    layer: 'HL7 v2.5 dictionary'
  }
];

/** The sample message used across the site, so every page shows the same one. */
export const sampleMessage = [
  'MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815081500||ORU^R01^ORU_R01|MSG00042|P|2.5',
  'PID|1||444333222^^^ACME&1.2.840.114398.1.100&ISO^MR||EVERYWOMAN^EVE^E||19620320|F',
  'OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|<200|N|||F'
].join('\n');
