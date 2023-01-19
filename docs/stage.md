

# information:
	Version: 0.9
	
	Note that there will be change in stage to stageless.

 * https://www.youtube.com/watch?v=h9FQDupcPlg&list=PLT_D88-MTFOPPl75g4WshL1Gx2bnGTUkz&index=4
```rust
app.add_startup_system_to_stage(StartupStage::PreStartup,assets_loading)
```
To load assets due to not set correct ways.
```
PreStartup
StartupStage
PostStartup
```