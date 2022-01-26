use syn::ItemFn;
use anyhow::Result;

struct CommandParams {

}

fn get_params(func: ItemFn) -> Result<Vec<CommandParams>> {
    for param in func.sig.inputs.iter() {
        param.
    }
}