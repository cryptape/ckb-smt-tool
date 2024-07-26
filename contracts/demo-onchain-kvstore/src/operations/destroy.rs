use crate::error::Result;

pub(crate) fn destroy(_index: usize) -> Result<()> {
    debug!("destroy the kvstore from inputs[{_index}]");
    Ok(())
}
