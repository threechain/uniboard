
import { Orchestrator } from "@holochain/tryorama";

import career from './referral/career';

let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
career(orchestrator);
orchestrator.run();



