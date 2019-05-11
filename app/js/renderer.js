
window.$ = window.jQuery = require('jquery')
window.Bootstrap = require('bootstrap')

const path = require("path");
const nScopeAPI = require(path.resolve('app/js/nScopeAPI'));
const nsAnalogOutputs = require(path.resolve('app/js/analogOutputs'));
const nsPulseOutput = require(path.resolve('app/js/pulseOutputs'));


function updatePowerUsage(state, usage)
{
    switch(state)
    {
        case 0:
        {
            $("#usb-status-bar, #usb-status").css("visibility","hidden");
            $(".nscope-usb-disconnected").css("visibility","hidden");
            $(".nscope-usb-power-fault").css("visibility","hidden");
            $(".nscope-usb-power-off").css("visibility","visible");
            updatePowerUsage.percentage = null;
            break;
        }
        case 1:
        {
            $(".nscope-usb-disconnected").css("visibility","hidden");
            $(".nscope-usb-power-off").css("visibility","hidden");
            $(".nscope-usb-power-fault").css("visibility","hidden");
            $("#usb-status-bar, #usb-status").css("visibility","visible");
            var percentage = usage*100/2.5;
            updatePowerUsage.percentage = updatePowerUsage.percentage*0.8+percentage*0.2 || percentage;
            $(".nscope-power-usage-bar .nscope-power-usage").css("width",updatePowerUsage.percentage+'%');
            $('#usb-status-bar, #usb-status').html((updatePowerUsage.percentage/100*2.5).toFixed(2)+' W');
            break;
        }
        case 2:
        case 3:
        {
            $("#usb-status-bar, #usb-status").css("visibility","hidden");
            $(".nscope-usb-disconnected").css("visibility","hidden");
            $(".nscope-usb-power-off").css("visibility","hidden");
            $(".nscope-usb-power-fault").css("visibility","visible");
            updatePowerUsage.percentage = null;
            break;
        }
        default:
        {
            $("#usb-status-bar, #usb-status").css("visibility","hidden");
            $(".nscope-usb-power-off").css("visibility","hidden");
            $(".nscope-usb-power-fault").css("visibility","hidden");
            $(".nscope-usb-disconnected").css("visibility","visible");
            updatePowerUsage.percentage = null;
            break;
        }
    }
    return usage;
}

function initPowerUsage()
{
    updatePowerUsage(-1, 0);
}

function monitorScope(){

    monitorScope.isOpen = monitorScope.isOpen || false;
    if(!monitorScope.isOpen)
    {
        if(nScopeAPI.open() == 0)
        {
            monitorScope.isOpen = true;
            nsPulseOutput.initInput();
        }
    }
    else
    {
        let state = nScopeAPI.get_power_state();
        let usage = nScopeAPI.get_power_usage();
        if(state < 0 || usage < 0)
        {
            monitorScope.isOpen = false;
        }
        updatePowerUsage(state, usage);
    }

}

$('.dropdown-menu.clickable').click(function(e) {
    e.stopPropagation();
});

$("label[contenteditable='true'").keypress(function(e) { 
    return false;
    // if(e.which == 13)
    // {

    // }   
    return e.which != 13; 
});




// Monitor the state of the nScope:
initPowerUsage();
monitorScope();
setInterval(monitorScope,10);


