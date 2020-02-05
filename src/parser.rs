use crate::decimal::Decimal;
use crate::ftcodec::{DateTime as PDateTime, ObjSnapshot, Snapshot, Snapshot_Field, Symbol};
use anyhow::Result;
use chrono::{DateTime, NaiveTime, TimeZone, Timelike, Utc};
use encoding_rs::GB18030;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};
use std::path::Path;

#[derive(Debug)]
struct ObjSnapshotC {
    snapshot: ObjSnapshot,
    changed: bool,
}

impl Default for ObjSnapshotC {
    fn default() -> Self {
        let mut snapshot = ObjSnapshot::new();
        snapshot.mut_lists().push({
            let mut s = Snapshot::new();
            s.mut_fields()
                .mut_value()
                .resize(Snapshot_Field::TOTAL as usize + 1, 0);
            s
        });
        ObjSnapshotC {
            snapshot,
            changed: true,
        }
    }
}

impl Deref for ObjSnapshotC {
    type Target = ObjSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl DerefMut for ObjSnapshotC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.snapshot
    }
}

fn parse_datetime(value: &str) -> Result<DateTime<Utc>> {
    const DATETIME_FORMAT: &str = "%Y%m%d-%H:%M:%S%.3f";
    Ok(Utc.datetime_from_str(value, DATETIME_FORMAT)?)
}

fn parse_time<Tz: TimeZone>(base: &DateTime<Tz>, value: &str) -> Result<DateTime<Tz>> {
    const TIME_FORMAT: &str = "%H:%M:%S%.3f";
    let t = NaiveTime::parse_from_str(value, TIME_FORMAT)?;
    Ok(base
        .date()
        .and_hms_nano(t.hour(), t.minute(), t.second(), t.nanosecond()))
}

pub struct MktdtParser {
    file_time: DateTime<Utc>,
    cached: HashMap<String, ObjSnapshotC>,
}

impl MktdtParser {
    pub fn new() -> Self {
        MktdtParser {
            file_time: Utc::now(),
            cached: Default::default(),
        }
    }

    pub fn parse_file<P: AsRef<Path>>(&mut self, filename: P) -> Result<()> {
        let rdr = encoding_rs_io::DecodeReaderBytesBuilder::new()
            .encoding(Some(&GB18030))
            .bom_sniffing(false)
            .build(File::open(filename)?);
        let mut input = BufReader::new(rdr);
        let mut line = String::new();
        while let Ok(n) = input.read_line(&mut line) {
            if n == 0 {
                break;
            }
            self.parse_line(line.trim_end())?;
            line.clear();
        }

        Ok(())
    }

    fn parse_line(&mut self, line: &str) -> Result<()> {
        let mut it = line.split("|");
        if let Some(id) = it.next() {
            match id {
                "HEADER" => self.parse_header(it)?,
                "MD001" => self.parse_md001(it)?,
                "MD002" => self.parse_md002(it)?,
                "MD003" => self.parse_md003(it)?,
                "MD004" => self.parse_md004(it)?,
                _ => {}
            }
        }
        Ok(())
    }

    fn parse_header<'a>(&mut self, mut it: impl Iterator<Item = &'a str>) -> Result<()> {
        if let Some(value) = it.nth(5) {
            self.file_time = parse_datetime(value)?;
        }
        Ok(())
    }

    fn parse_md001<'a>(&mut self, it: impl Iterator<Item = &'a str>) -> Result<()> {
        const FIELDS: [Snapshot_Field; 8] = [
            Snapshot_Field::VOLUME,
            Snapshot_Field::AMOUNT,
            Snapshot_Field::PRE_CLOSE_PRICE,
            Snapshot_Field::OPEN_PRICE,
            Snapshot_Field::HIGH_PRICE,
            Snapshot_Field::LOW_PRICE,
            Snapshot_Field::PRICE,
            Snapshot_Field::CLOSE_PRICE,
        ];

        self.parse_md00(it, &FIELDS)
    }

    fn parse_md002<'a>(&mut self, it: impl Iterator<Item = &'a str>) -> Result<()> {
        const FIELDS: [Snapshot_Field; 28] = [
            Snapshot_Field::VOLUME,
            Snapshot_Field::AMOUNT,
            Snapshot_Field::PRE_CLOSE_PRICE,
            Snapshot_Field::OPEN_PRICE,
            Snapshot_Field::HIGH_PRICE,
            Snapshot_Field::LOW_PRICE,
            Snapshot_Field::PRICE,
            Snapshot_Field::CLOSE_PRICE,
            Snapshot_Field::BID1_PRICE,
            Snapshot_Field::BID1_VOLUME,
            Snapshot_Field::ASK1_PRICE,
            Snapshot_Field::ASK1_VOLUME,
            Snapshot_Field::BID2_PRICE,
            Snapshot_Field::BID2_VOLUME,
            Snapshot_Field::ASK2_PRICE,
            Snapshot_Field::ASK2_VOLUME,
            Snapshot_Field::BID3_PRICE,
            Snapshot_Field::BID3_VOLUME,
            Snapshot_Field::ASK3_PRICE,
            Snapshot_Field::ASK3_VOLUME,
            Snapshot_Field::BID4_PRICE,
            Snapshot_Field::BID4_VOLUME,
            Snapshot_Field::ASK4_PRICE,
            Snapshot_Field::ASK4_VOLUME,
            Snapshot_Field::BID5_PRICE,
            Snapshot_Field::BID5_VOLUME,
            Snapshot_Field::ASK5_PRICE,
            Snapshot_Field::ASK5_VOLUME,
        ];

        self.parse_md00(it, &FIELDS)
    }

    fn parse_md003<'a>(&mut self, it: impl Iterator<Item = &'a str>) -> Result<()> {
        self.parse_md002(it)
    }

    fn parse_md004<'a>(&mut self, it: impl Iterator<Item = &'a str>) -> Result<()> {
        const FIELDS: [Snapshot_Field; 30] = [
            Snapshot_Field::VOLUME,
            Snapshot_Field::AMOUNT,
            Snapshot_Field::PRE_CLOSE_PRICE,
            Snapshot_Field::OPEN_PRICE,
            Snapshot_Field::HIGH_PRICE,
            Snapshot_Field::LOW_PRICE,
            Snapshot_Field::PRICE,
            Snapshot_Field::CLOSE_PRICE,
            Snapshot_Field::BID1_PRICE,
            Snapshot_Field::BID1_VOLUME,
            Snapshot_Field::ASK1_PRICE,
            Snapshot_Field::ASK1_VOLUME,
            Snapshot_Field::BID2_PRICE,
            Snapshot_Field::BID2_VOLUME,
            Snapshot_Field::ASK2_PRICE,
            Snapshot_Field::ASK2_VOLUME,
            Snapshot_Field::BID3_PRICE,
            Snapshot_Field::BID3_VOLUME,
            Snapshot_Field::ASK3_PRICE,
            Snapshot_Field::ASK3_VOLUME,
            Snapshot_Field::BID4_PRICE,
            Snapshot_Field::BID4_VOLUME,
            Snapshot_Field::ASK4_PRICE,
            Snapshot_Field::ASK4_VOLUME,
            Snapshot_Field::BID5_PRICE,
            Snapshot_Field::BID5_VOLUME,
            Snapshot_Field::ASK5_PRICE,
            Snapshot_Field::ASK5_VOLUME,
            Snapshot_Field::PRE_IOPV,
            Snapshot_Field::IOPV,
        ];

        self.parse_md00(it, &FIELDS)
    }

    fn parse_md00<'a>(
        &mut self,
        mut it: impl Iterator<Item = &'a str>,
        fields: &[Snapshot_Field],
    ) -> Result<()> {
        let snapshot = match it.next() {
            Some(code) => {
                let snapshot = self
                    .cached
                    .entry(code.to_string())
                    .or_insert(Default::default());

                if snapshot.symbol.is_none() {
                    snapshot.set_symbol(Symbol {
                        market: 0,
                        code: code.into(),
                        ..Symbol::default()
                    });
                }

                snapshot
            }
            None => bail!("invalid row"),
        };

        if let Some(value) = it.next() {
            if snapshot.name.is_empty() {
                snapshot.set_name(value.into());
            }
        } else {
            bail!("invalid row")
        }

        for field in fields.iter() {
            if let Some(value) = it.next() {
                let row = snapshot.lists.first_mut().unwrap();
                let value = Decimal::parse(value);
                let old_value = row.mut_fields().value[*field as usize];
                let new_value = value.to_i64();
                row.mut_fields().value[*field as usize] = new_value;
                snapshot.changed = old_value != new_value;
            } else {
                bail!("invalid row")
            }
        }

        it.next(); // TradingPhaseCode

        if let Some(value) = it.next() {
            snapshot
                .lists
                .first_mut()
                .unwrap()
                .set_timestamp(PDateTime {
                    value: parse_time(&self.file_time, value)?.timestamp_millis(),
                    ..PDateTime::default()
                });
        }

        Ok(())
    }
}
