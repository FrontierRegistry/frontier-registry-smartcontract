# Frontier-Registry-Smartcontract

For Soroban Build - w/Support from Stellar Development Foundation as a result of performance July 2024 Kickstarter Bootcamp
Participants in Bootcamp, Paige Donner, Founder, Frontier Registry, owner IoT Logistics, LLC 
Contact:  
Discord @artemisp_3d   Tg @paige3d

[IoT Logistics, LLC](https://iotlogistics.international) 

Existing dApp   https://frontier-dapp.netlify.app

Info Page  https://frontier-registry.carrd.co  

[Explainer Video](https://www.youtube.com/watch?v=lAxw35kjUzI)


**Lead Dev for Soroban Build Toru Ichikawa**  
Contact:  [Portfolio](https://spectrecoder.vercel.app)   Discord @spectrecoder
Github @spectrecoder 


### **SEE BELOW FOR WEEKLY DEVELOPER'S LOG**

# **SECTION ONE** THIS Section:  Project Resources & Overview 

This MVP build phase will last 5 weeks with no more than 6 weeks maximum.

The App is now listed on  [Magic Store](https://magic.store/app/frontier-registry) with a scheduled release date of Sept. 15th


**The main goals are to**
1. Adapt Frontier Registry to Stellar Mainnet using Soroban Smart Contracts
2. Integrate Freighter wallet - using plug-ins noted in the technical architecture
3. Automate a certification system so that the users can upload their research, publish it on chain
and receive a Certification document (NOTED In the Pitch Deck slide)
4. We need to have a function that shows the user what is processing, when.
5. We will deliver a working tool by the 5th week.
6. The next phase (Build Phase 2) will be AI integration for DATA PROVENANCE (AFTER these next 6 weeks)


## **ADDITIONAL RESOURCES**

 [Figma File - Frontier Registry](https://www.figma.com/board/iha5vwoCzCyAkxXQXp13rs/Frontier-Registry-x-STELLAR-StartUp-Camp?node-id=0-1) (from Stellar Bootcamp)


 [Stellar Simple SigniN](https://github.com/bigger-tech/simple-stellar-signer)

[More tools](https://stellar-startup-camp.biggertech.co/fundamentals-of-integrating-with-stellar/tools-and-resources) for Stellar Integration:


Shared Folder
Please find the technical architecture documents in the shared folder HERE:
Frontier Registry - Soroban Build : Kickstarter Award

The Github Repo can be found here and also on the link below.
https://github.com/FrontierRegistry/Soroban






### <ins>Developer Log</ins> 

**August 23 - August 31st** 


### <ins>Report 1</ins> 

- Built a project structure for the Frontier registry project
  
- Complete the project structure by dividing it into a contract for
creating and managing NFTs and a contract for storing research data
and NFT IDs on the chain

- Built a data structure for NFT management and write NFT management functions
  
- Built an NFT data structure Build Admin, Datakey, DatakeyMetadata,
and DataKeyEnumerable data structures

- Wrote an NFT initialization function Exception handling to determine
whether it is already initialized Register on the chain if it is the
first time

- Wrote a mint function Exception handling to determine whether a
token id exists Register token data on the chain



