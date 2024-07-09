#[ic_cdk_macros::init]
fn init() {
    ic_cdk::print("Smart contract initialized");
}

#[ic_cdk_macros::update]
async fn execute() {
    ic_cdk::print("Smart contract executed");
}
