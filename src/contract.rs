#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg, MemberResponse};
use crate::state::{State, STATE, Member, MEMBERS};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let sender = info.sender.clone();

    let curr_time = _env.block.time;

    println!("info.sender:: {:?},  time :{}", sender.as_str(), curr_time);

    let state = State {
        count: msg.count,
        owner: sender,
        message : String::from("This is a state 1"),
        updated : curr_time, 
    };
       
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps, _env),
        ExecuteMsg::Reset { count } => try_reset(deps, _env, info, count),
        ExecuteMsg::AddNewMember { key, name, age} => add_member(deps, _env, key, name, age),
        ExecuteMsg::UpdateMember { key, name, age} => update_member(deps, _env, key, name, age),
        ExecuteMsg::DeleteMember { key } => delete_member(deps, _env, key ),
        //_ => { println!("Currently do nothing, will update this"); Ok( Response::default()) },
    }
}



pub fn try_increment(deps: DepsMut, _env: Env) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        state.message = format!("Counter incremented {}", state.count);
        state.updated = _env.block.time; 
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}
pub fn try_reset(deps: DepsMut, _env : Env, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        state.message = format!("Counter reset :{}", state.count);
        state.updated = _env.block.time;

        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetMember {key} => to_binary(&query_member(deps, key)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count, message : state.message, 
        owner : state.owner.into_string(), updated : state.updated.nanos() / 1_000_000  })
}

fn query_member ( deps : Deps, key : String ) -> StdResult<MemberResponse> {

    let mem = MEMBERS.key(key.as_str());

    let stored_mem = mem.may_load(deps.storage)?;
    if stored_mem == None {

        return Ok(MemberResponse{ found : false , member : None});
    }
    
    Ok(MemberResponse{ found : true , member : stored_mem })
}


pub fn add_member (_deps: DepsMut, _env : Env, _key : String , 
    _name : String , _age : i8) -> Result<Response, ContractError>  {
   
   
    let new_member = Member { name : _name, age : _age, date_joined : _env.block.time };

    println!("New member : {:?}", new_member);

    let mem = MEMBERS.key(_key.as_str());
    
    let empty = mem.may_load(_deps.storage)?;
    assert_eq!(None, empty); // just to check if it already exists by the key

    MEMBERS.save(_deps.storage,_key.as_str(),&new_member)?;

    Ok(Response::new().add_attribute("method", "add_member"))

}



pub fn update_member (deps: DepsMut, _env : Env, key : String , 
    name : String , age : i8) -> Result<Response, ContractError>  {
   
    let updated_member = Member { name : name, age : age, date_joined : _env.block.time };
   

    let mem = MEMBERS.key(key.as_str());

    let stored_mem = mem.may_load(deps.storage)?;
  
    if stored_mem == None {

        return Err(ContractError::MemberNotFoundError{});
    }

    println!("Updated member : {:?}", updated_member);
   
    
    MEMBERS.save(deps.storage,key.as_str() ,&updated_member)?;

    
    Ok(Response::new().add_attribute("method", "member updated!"))

}

pub fn delete_member  (deps: DepsMut, _env : Env, key : String ) -> Result<Response, ContractError>  {
   
    let mem = MEMBERS.key(key.as_str());

    let stored_mem = mem.may_load(deps.storage)?;
  
    if stored_mem == None {

        return Err(ContractError::MemberNotFoundError{});
    }

    MEMBERS.remove(deps.storage, key.as_str());
    
    Ok(Response::new().add_attribute("method", "member.deleted!"))


}