# Automated Transaction State Graph

This is current prototype for the automated transaction state graph.

This data structure will serve as the backbone of the automated transaction sovereign rollup I'm currently writing about [here](https://respected-atlasaurus-022.notion.site/Automated-Transaction-DeFi-Sovereign-Rollup-1a5bd21906d34f349d468ce64a7bf788).

This data structure is responsible for keeping the current validity state of the transactions it stores. 
The goal is to have constant distinction between two two disjoint sets, the valid and invalid transaction sets. 

Down the line, it would be useful to include a priority ordering within the sets based on the transactions' associated fees. 

Stay tuned for more
