use {Error, Matrix, Project, Result, Scan, Vector};
use nalgebra::Eye;
use std::collections::HashMap;
use std::path::Path;

/// A scan position.
#[derive(Clone, Debug)]
pub struct ScanPosition {
    name: String,
    pop: Matrix,
    scans: HashMap<String, Scan>,
    sop: Matrix,
}

impl ScanPosition {
    /// Creates a new scan position from the provided path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// let path = "data/project.RiSCAN/SCANS/SP01";
    /// let scan_position = ScanPosition::from_path(path).unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ScanPosition> {
        let fullpath = path.as_ref().canonicalize()?;
        let mut path_buf = fullpath.clone();
        loop {
            if let Ok(project) = Project::from_path(&path_buf) {
                let scans_path = project.path().unwrap().join("SCANS");
                let subpath = fullpath.strip_prefix(&scans_path)
                    .map_err(|_| Error::NotAScanPosition(path.as_ref().to_path_buf()))?;
                if let Some(scan_position) = subpath.iter().next() {
                    return project.scan_position(&scan_position.to_string_lossy())
                        .map(|scan_position| scan_position.clone())
                        .ok_or(Error::NotAScanPosition(path.as_ref().to_path_buf()));
                } else {
                    return Err(Error::NotAScanPosition(path.as_ref().to_path_buf()));
                }
            }
            if !path_buf.pop() {
                break;
            }
        }
        Err(Error::NotAProject(path.as_ref().to_path_buf()))
    }

    /// Creates a new scan position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// let scan_position = ScanPosition::new();
    /// ```
    pub fn new() -> ScanPosition {
        ScanPosition {
            name: String::new(),
            pop: Matrix::new_identity(4),
            scans: HashMap::new(),
            sop: Matrix::new_identity(4),
        }
    }

    /// Returns this scan position's name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// let mut scan_position = ScanPosition::new();
    /// scan_position.set_name("ScanPos001");
    /// assert_eq!("ScanPos001", scan_position.name());
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets this scan position's name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// # let mut scan_position = ScanPosition::new();
    /// scan_position.set_name("New name");
    /// ```
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Returns this scan position's SOP.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// # let scan_position = ScanPosition::new();
    /// let sop = scan_position.sop();
    /// ```
    pub fn sop(&self) -> Matrix {
        self.sop
    }

    /// Sets this scan position's SOP.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::{ScanPosition, Matrix};
    /// # let mut scan_position = ScanPosition::new();
    /// scan_position.set_sop(Matrix::new(1., 0., 0., 1.,
    ///                                   0., 1., 0., 2.,
    ///                                   0., 0., 1., 3.,
    ///                                   0., 0., 0., 1.));
    /// ```
    pub fn set_sop(&mut self, sop: Matrix) {
        self.sop = sop;
    }

    /// Returns this scan position's POP.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// # let scan_position = ScanPosition::new();
    /// let pop = scan_position.pop();
    /// ```
    pub fn pop(&self) -> Matrix {
        self.pop
    }

    /// Sets this scan position's POP.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::{ScanPosition, Matrix};
    /// # let mut scan_position = ScanPosition::new();
    /// scan_position.set_pop(Matrix::new(1., 0., 0., 1.,
    ///                                   0., 1., 0., 2.,
    ///                                   0., 0., 1., 3.,
    ///                                   0., 0., 0., 1.));
    /// ```
    pub fn set_pop(&mut self, pop: Matrix) {
        self.pop = pop;
    }

    /// Converts SOCS coordinates to GLCS coordinates.
    ///
    /// Convert (0., 0., 0.) to get the scanner's origin in GLCS.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// # let scan_position = ScanPosition::new();
    /// let (x, y, z) = scan_position.socs_to_glcs((1., 2., 3.));
    /// ```
    pub fn socs_to_glcs(&self, (x, y, z): (f64, f64, f64)) -> (f64, f64, f64) {
        let glcs = self.pop * self.sop * Vector::new(x, y, z, 1.);
        (glcs.x, glcs.y, glcs.z)
    }

    /// Returns a reference to the scan with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// let scan_position = ScanPosition::from_path("data/project.RiSCAN/SCANS/SP01").unwrap();
    /// let scan = scan_position.scan("151120_150404").unwrap();
    /// ```
    pub fn scan(&self, name: &str) -> Option<&Scan> {
        self.scans.get(name)
    }

    /// Adds a scan.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::{ScanPosition, Scan};
    /// let mut scan_position = ScanPosition::new();
    /// scan_position.add_scan(Scan::new());
    /// ```
    pub fn add_scan(&mut self, scan: Scan) {
        self.scans.insert(scan.name().to_string(), scan);
    }
}

#[cfg(test)]
mod tests {
    use Project;

    #[test]
    fn scan_position_glcs() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let scan_position = project.scan_position("SP01").unwrap();
        let (x, y, z) = scan_position.socs_to_glcs((1., 2., 3.));
        assert!((-515633.63 - x).abs() < 1e-2);
        assert!((-5519674.02 - y).abs() < 1e-2);
        assert!((3143445.58 - z).abs() < 1e-2);
    }
}
