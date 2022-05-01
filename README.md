This library adds macro to get compile time counters.

It uses a thread_local variables & procedural macros to add state to rust macros.

I have been unable to find any explanations of the parallization model of rustc,
I find it unlikely that multiple threads will be modifying a source file at the
same time.
Thus we are using thread_local.

## Warning
I'm not certain about the stability or safety of this, so I would not
recomend this for use in serious projects.
