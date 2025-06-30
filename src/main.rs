use nalgebra::DMatrix; // dynamically sized column-major matrix
use faer::Mat; // FIXME confirm layout
use numpy::ndarray::ArrayView2;
use numpy::ndarray::ShapeBuilder; // Required for strides() on shape
use numpy::{PyArray2, ToPyArray};
use pyo3::prelude::*;

pub trait MatrixToPy<'py> {
    fn to_pyarray_view(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>>;
}

impl<'py> MatrixToPy<'py> for DMatrix<f64> {
    fn to_pyarray_view(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        let view = unsafe {
            ArrayView2::from_shape_ptr(
                self.shape().strides(self.strides()),
                self.as_ptr()
            )
        };
        view.to_pyarray(py).into()
    }
}

impl<'py> MatrixToPy<'py> for Mat<f64> {
    fn to_pyarray_view(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        let strides = (self.row_stride() as usize, self.col_stride() as usize);
        let view = unsafe {
            ArrayView2::from_shape_ptr(
                self.shape().strides(strides),
                self.as_ptr()
            )
        };
        view.to_pyarray(py).into()
    }
}

fn nalgebra_test<'py>(py: Python<'py>) -> Bound<'py, PyArray2<f64>>{
    let mut mat: DMatrix<f64> = DMatrix::zeros(7, 13);
    mat[(2, 5)] = 5.0; // Example value to test orientation
    mat.to_pyarray_view(py)
}

fn faer_test<'py>(py: Python<'py>) -> Bound<'py, PyArray2<f64>>{
    let mut mat: Mat<f64> = Mat::zeros(7, 13);
    *mat.get_mut(2, 5) = 5.0; // Example value to test orientation
    mat.to_pyarray_view(py)
}

// TODO
// 1. lifetime checking, best practice for safety:
//  - Only return the array inside Python::with_gil, and do not let it outlive its source Rust object
//  - If returning to Python permanently, wrap the backing buffer in a PyCapsule or move to an owned Array2
//  - To verify: try accessing the Python array after dropping the Rust backing data—if it segfaults or panics, you’ve overstepped the lifetime.
// 2. Ensure no internal data copies (zero-copy confidence)
//  - Valgrind for catching memory leaks and tracking heap allocations, detecting excess copies during execution
//    `valgrind --tool=massif ./target/debug/your_binary`
//    `ms_print massif.out.<pid>`
//  - "miri" rchecks for undefined behavior, including invalid memory access and double frees
//    `cargo +nightly miri run`
// 3. General outstanding work:
//  - Lifetime of data (ensure objects that created arrays are kept in scope/PyCell/problem)
fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys").unwrap();
        println!("sys.prefix = {:?}", sys.getattr("prefix").unwrap());
        println!("sys.executable = {:?}", sys.getattr("executable").unwrap());

        for (key, value) in std::env::vars() {
            println!("{key}: {value}");
        }

        let faer_arr = faer_test(py);
        println!("faer_arr:\n{}\n", faer_arr.repr().unwrap().to_str().unwrap());

        let nalgebra_arr = nalgebra_test(py);
        println!("nalgebra_arr:\n{}\n", nalgebra_arr.repr().unwrap().to_str().unwrap());

        Ok(())
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Dummy test more for user to check that tests can be run and debug
    /// using hover buttons in VSCode UI. This requires `.cargo/config.toml` and
    /// `"lldb.launch.env"` have been set up; see README.md for details.
    #[test]
    fn test_env_numpy_correct_in_gui() {
        Python::with_gil(|py| {
            faer_test(py);
            nalgebra_test(py);
        });
    }
}