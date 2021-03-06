extern crate uuid;

use self::uuid::Uuid;
use types::{RawFromSql, ToSql, RawToSql, Type};
use Error;
use Result;

impl RawFromSql for Uuid {
    fn raw_from_sql<R: Reader>(_: &Type, raw: &mut R) -> Result<Uuid> {
        match Uuid::from_bytes(&*try!(raw.read_to_end())) {
            Some(u) => Ok(u),
            None => Err(Error::BadData),
        }
    }
}

from_raw_from_impl!(Type::Uuid; Uuid);

impl RawToSql for Uuid {
    fn raw_to_sql<W: Writer>(&self, _: &Type, w: &mut W) -> Result<()> {
        Ok(try!(w.write_all(self.as_bytes())))
    }
}

to_raw_to_impl!(Type::Uuid; Uuid);
