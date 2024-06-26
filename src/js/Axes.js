import {idFromCh} from './Utils.js'

export const colors = {
    "Ch1": 'rgb(52,210,146)',
    "Ch2": 'rgb(246,216,97)',
    "Ch3": 'rgb(58,176,226)',
    "Ch4": 'rgb(233,102,86)',
    "Trigger": 'rgb(255,255,255)',
};

export const text_colors = {
    "Ch1": "black",
    "Ch2": "black",
    "Ch3": "white",
    "Ch4": "white",
    "Trigger": "black",
}

export var ranges = {
    "Ch1": [-5, 5],
    "Ch2": [-5, 5],
    "Ch3": [-5, 5],
    "Ch4": [-5, 5],
};


export function channel_axis(ch) {
    return {
        overlaying: 'y',
        range: ranges[ch],
        fixedrange: true,
        tickfont: {color: colors[ch]},
        showticklabels: false,
        zeroline: false,
        showgrid: false,
        position: (idFromCh(ch)-1)/3.0,
    }
}

export function update() {
    return {
        yaxis2: channel_axis("Ch1"),
        yaxis3: channel_axis("Ch2"),
        yaxis4: channel_axis("Ch3"),
        yaxis5: channel_axis("Ch4"),
    }
}